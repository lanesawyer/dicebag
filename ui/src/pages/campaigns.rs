use graphql_client::GraphQLQuery;
use serde::Deserialize;
use serde_json::json;
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, use_effect, use_state, Callback, Html};
use yew_router::prelude::Link;

use crate::{
    components::{Button, TextField},
    navigation::AppRoute,
    services::{self, campaigns_query, CampaignsQuery, GraphQLResponse, new_campaign_mutation::{self, NewCampaign}, NewCampaignMutation}
};

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Campaign {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CampaignList {
    pub campaigns: Vec<Campaign>,
}

#[derive(Clone)]
struct MyGraphQLRequest {
    pub data: Option<Vec<Campaign>>,
    pub error: Option<String>,
}

fn use_query() -> MyGraphQLRequest {
    let state = use_state(|| MyGraphQLRequest {
        data: None,
        error: None,
    });

    let effect_state = state.clone();

    use_effect(move || {
        // TODO: Is there a better way to keep it from making infinite network requests?
        if effect_state.data.is_none() || effect_state.error.is_none() {
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
        }

        // TODO: Figure out if I need to return something else here
        || ()
    });

    (*state).clone()
}

#[function_component(CampaignsPage)]
pub fn campaigns_page() -> Html {
    let query = use_query();

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

    // let on_submit = {
    //     use_effect(move || {
    //         // TODO: Is there a better way to keep it from making infinite network requests?
    //         if effect_state.data.is_none() || effect_state.error.is_none() {
    //             spawn_local(async move {
    //                 let variables = new_campaign_mutation::Variables {
    //                     new_campaign: NewCampaign {
    //                         name: *(new_name).clone(),
    //                         description: *(new_description).clone(),
    //                     }
    //                 };
    //                 let request_body = NewCampaignMutation::build_query(variables);
    //                 let request_json = &json!(request_body);

    //                 let request = services::build_request(request_json).await;
    //                 if let Ok(response) = request {
    //                     let json = response.json::<GraphQLResponse<CampaignList>>().await;
    //                     match json {
    //                         Ok(responser) => effect_state.set(MyGraphQLRequest {
    //                             data: Some(responser.data.campaigns),
    //                             error: None,
    //                         }),
    //                         Err(error) => effect_state.set(MyGraphQLRequest {
    //                             data: None,
    //                             error: Some(error.to_string()),
    //                         }),
    //                     }
    //                 }
    //             });
    //         }

    //         // TODO: Figure out if I need to return something else here
    //         || ()
    //     });
    // };

    html! {
        <section class="list-page">
            {
                if let Some(campaigns) = &query.data {
                    campaigns.iter().map(|c| view_campaign(c)).collect::<Html>()
                } else {
                    // TODO: Character skeleton while loading
                    html! { <></> }
                }
            }
            <div class="list-item add-character-panel">
                <TextField label="Name" value={(*new_name).clone()} on_change={on_change_name} />
                <TextField label="Description" value={(*new_description).clone()} on_change={on_change_description} />
                <Button label="Create" icon_name={"plus".to_string()} on_click={Callback::from(move |_| new_description.set("hi".to_string()))} />
            </div>
        </section>
    }
}

fn view_campaign(campaign: &Campaign) -> Html {
    html! {
        <div class="list-item">
            <Link<AppRoute> to={AppRoute::Campaign { id: campaign.id }}>
                <div class="list-item character-panel">
                    <span class="character-name">{campaign.name.clone()}</span>
                    // <span class="character-class">{campaign.description.clone()}</span>
                </div>
            </Link<AppRoute>>
        </div>
    }
}
