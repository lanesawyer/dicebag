use std::collections::HashMap;

use graphql_client::GraphQLQuery;
use serde_json::json;
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, use_effect_with_deps, use_state, Callback, Properties};
use yew_router::hooks::use_history;
use yew_router::history::History;

use crate::{
    components::{Button, ButtonType, Icon},
    pages::{
        campaign::initiative_tracker::{InitiativeInfo, InitiativeTracker},
        character_sheet::sheet::{Character, CharacterList},
    },
    services::{self, characters_query, CharactersQuery, delete_campaign_mutation, DeleteCampaignMutation, GraphQLResponse}, navigation::AppRoute,
};

#[derive(Clone)]
struct MyGraphQLRequest {
    pub data: Option<Vec<Character>>,
    pub error: Option<String>,
}

fn use_characters_query() -> MyGraphQLRequest {
    let state = use_state(|| MyGraphQLRequest {
        data: None,
        error: None,
    });

    let effect_state = state.clone();

    use_effect_with_deps(
        move |_| {
            spawn_local(async move {
                let variables = characters_query::Variables {};
                let request_body = CharactersQuery::build_query(variables);
                let request_json = &json!(request_body);

                let request = services::build_request(request_json).await;

                if let Ok(response) = request {
                    let json = response.json::<GraphQLResponse<CharacterList>>().await;
                    match json {
                        Ok(responser) => effect_state.set(MyGraphQLRequest {
                            data: Some(responser.data.characters),
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

#[derive(Properties, Clone, PartialEq)]
pub struct CampaignProps {
    pub id: i64,
}

#[function_component(CampaignPage)]
pub fn campaign_page(props: &CampaignProps) -> Html {
    let characters = use_characters_query();
    let history = use_history().expect("history should be available");

    if characters.data.is_none() {
        return html! { <></> };
    }

    let delete_campaign = {
        let delete_id = props.id;
        Callback::once(move |_| {
            spawn_local(async move {
                let variables = delete_campaign_mutation::Variables {
                    delete_id,
                };
                let request_body = DeleteCampaignMutation::build_query(variables);
                let request_json = &json!(request_body);

                let request = services::build_request(request_json).await;

                if let Ok(response) = request {
                    let json = response.json::<GraphQLResponse<bool>>().await;
                    match json {
                        Ok(responser) => {
                            // TODO: Redirect here instead of assuming it works
                            // history.push(AppRoute::Campaigns);
                        },
                        Err(error) => (),
                    }
                }
            });
            // TODO: Figure out how to do a redirect within the local thread
            history.push(AppRoute::Campaigns);
        })
    };

    let characters: HashMap<String, InitiativeInfo> = characters
        .data
        .unwrap_or_default()
        .iter()
        .map(|character| {
            (
                character.name.clone(),
                InitiativeInfo {
                    name: character.name.clone(),
                    initiative: 1,
                },
            )
        })
        .collect();

    html! {
        <>
            <h1>
                { format!("Campaign number {}!", props.id) }
            </h1>
            <section id="settings">
                <Icon name="cog" />
                <Button label="Delete" icon_name={"trash".to_string()} button_type={ButtonType::Danger} on_click={delete_campaign} />
            </section>
            <InitiativeTracker {characters} />
        </>
    }
}
