use graphql_client::GraphQLQuery;
use serde::Deserialize;
use serde_json::json;
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, use_effect_with_deps, use_state, Callback, Html};
use yew_router::prelude::Link;

use crate::{
    components::{Button, TextField},
    navigation::AppRoute,
    services::{
        self, campaigns_query,
        new_campaign_mutation::{self, NewCampaign},
        CampaignsQuery, GraphQLResponse, NewCampaignMutation,
    },
};

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Campaign {
    pub id: i64,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CampaignList {
    pub campaigns: Vec<Campaign>,
}

#[derive(Clone)]
struct MyGraphQLRequest {
    pub data: Option<Vec<Campaign>>,
    pub error: Option<String>,
}

fn use_campaigns_query() -> MyGraphQLRequest {
    let state = use_state(|| MyGraphQLRequest {
        data: None,
        error: None,
    });

    let effect_state = state.clone();

    use_effect_with_deps(
        move |_| {
            spawn_local(async move {
                let variables = campaigns_query::Variables {};
                let request_body = CampaignsQuery::build_query(variables);
                let request_json = &json!(request_body);

                let request = services::build_request(request_json).await;
                if let Ok(response) = request {
                    let json = response.json::<GraphQLResponse<CampaignList>>().await;
                    match json {
                        Ok(responser) => effect_state.set(MyGraphQLRequest {
                            data: Some(responser.data.campaigns),
                            error: None,
                        }),
                        Err(error) => effect_state.set(MyGraphQLRequest {
                            data: None,
                            error: Some(error.to_string()),
                        }),
                    }
                }
            });

            || ()
        },
        (),
    );

    (*state).clone()
}

// TODO: Do a more generalized query function
// #[derive(Clone)]
// struct MySmallerGraphQLRequest<T> {
//     pub data: Option<T>,
//     pub error: Option<String>,
// }

// fn use_smaller_query<'a, T>(request_json: &serde_json::Value) -> MySmallerGraphQLRequest<T>
// where T: Deserialize<'a> + Clone {
//     let state = use_state(|| MySmallerGraphQLRequest {
//         data: None,
//         error: None,
//     });

//     let effect_state = state.clone();

//     let request_json = request_json.clone();
//     use_effect_with_deps(move |_| {
//         spawn_local(async move {
//             let request = services::build_request(&request_json).await;
//             if let Ok(response) = request {
//                 let json = response.json::<GraphQLResponse<T>>().await;
//                 match json {
//                     Ok(responser) => effect_state.set(MySmallerGraphQLRequest {
//                         data: Some(responser.data),
//                         error: None,
//                     }),
//                     Err(error) => effect_state.set(MySmallerGraphQLRequest {
//                         data: None,
//                         error: Some(error.to_string()),
//                     }),
//                 }
//             }
//         });

//         || ()
//     }, ());

//     (*state).clone()
// }

#[function_component(CampaignsPage)]
pub fn campaigns_page() -> Html {
    let query = use_campaigns_query();

    let new_name = use_state(|| "".to_string());
    let new_description = use_state(|| "".to_string());

    let on_change_name = {
        let new_name = new_name.clone();
        Callback::from(move |input| new_name.set(input))
    };

    let on_change_description = {
        let new_description = new_description.clone();
        Callback::from(move |input| new_description.set(input))
    };

    let on_submit = {
        let submit_name = new_name.clone();
        let submit_description = new_description.clone();

        Callback::from(move |_| {
            let submit_name = submit_name.clone();
            let submit_description = submit_description.clone();

            spawn_local(async move {
                let submit_name = &*(submit_name).clone();
                let submit_description = &*(submit_description).clone();
                let variables = new_campaign_mutation::Variables {
                    new_campaign: NewCampaign {
                        name: submit_name.to_string(),
                        // TODO: Figure out a better way to use an Option through the whole component
                        description: Some(submit_description.to_string()),
                    },
                };
                let request_body = NewCampaignMutation::build_query(variables);
                let request_json = &json!(request_body);

                let request = services::build_request(request_json).await;
                if let Ok(response) = request {
                    let json = response.json::<GraphQLResponse<bool>>().await;
                    match json {
                        Ok(_responser) => (),
                        Err(_error) => (),
                    }
                }
            });
        })
    };

    html! {
        <section class="list-page">
            <>{query.error.unwrap_or_else(|| "".to_string())}</>
            {
                if let Some(campaigns) = &query.data {
                    campaigns.iter().map(view_campaign).collect::<Html>()
                } else {
                    // TODO: Character skeleton while loading
                    html! { <></> }
                }
            }
            <div class="list-item add-character-panel">
                <TextField label="Name" value={(*new_name).clone()} on_change={on_change_name} />
                <TextField label="Description" value={(*new_description).clone()} on_change={on_change_description} />
                <Button label="Create" icon_name={"plus".to_string()} on_click={on_submit} />
            </div>
        </section>
    }
}

fn view_campaign(campaign: &Campaign) -> Html {
    html! {
        <div class="list-item">
            <Link<AppRoute> to={AppRoute::Campaign { id: campaign.id }}>
                <div class="list-item character-panel">
                    <div class="campaign-info">
                        <div class="character-name">{campaign.name.clone()}</div>
                        <div class="character-class">{campaign.description.clone()}</div>
                    </div>
                    <div></div> // Used for flex justify effect and future image
                    <div class="characters">
                        { "Characters here eventually" }
                    </div>
                </div>
            </Link<AppRoute>>
        </div>
    }
}
