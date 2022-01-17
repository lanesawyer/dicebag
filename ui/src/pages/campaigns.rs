use graphql_client::GraphQLQuery;
use serde::Deserialize;
use serde_json::json;
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, use_effect, use_state, Callback, Html};
use yew_router::prelude::Link;

use crate::{
    components::{Button, TextField},
    navigation::AppRoute,
    services::{self, campaigns_query, CampaignsQuery, GraphQLResponse},
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
        if effect_state.data.is_none() {
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
                        Err(_error) => todo!(),
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

    let onchange_name = {
        let new_name = new_name.clone();
        Callback::from(move |input| new_name.set(input))
    };

    let onchange_description = {
        let new_description = new_description.clone();
        Callback::from(move |input| new_description.set(input))
    };

    html! {
        // TODO: Make list page panels generic CSS
        <section id="characters-page">
            <div id="characters">
                {
                    if let Some(campaigns) = &query.data {
                        campaigns.iter().map(|c| view_campaign(c)).collect::<Html>()
                    } else {
                        // TODO: Character skeleton while loading
                        html! { <></> }
                    }
                }
                <div class="add-character-panel">
                    <TextField label="Name" value={(*new_name).clone()} on_change={onchange_name} />
                    <TextField label="Description" value={(*new_description).clone()} on_change={onchange_description} />
                    <Button label="Create" icon_name={"plus".to_string()} on_click={Callback::from(move |_| new_description.set("hi".to_string()))} />
                </div>
            </div>
        </section>
    }
}

fn view_campaign(campaign: &Campaign) -> Html {
    html! {
        <Link<AppRoute> to={AppRoute::Campaign { id: campaign.id }}>
            <div class="character-panel">
                <span class="character-name">{campaign.name.clone()}</span>
                // <span class="character-class">{campaign.description.clone()}</span>
            </div>
        </Link<AppRoute>>
    }
}
