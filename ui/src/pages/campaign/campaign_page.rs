use std::collections::HashMap;

use graphql_client::GraphQLQuery;
use serde_json::json;
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, use_effect_with_deps, use_state, Callback, Properties};

use crate::{
    pages::{
        campaign::initiative_tracker::{InitiativeInfo, InitiativeTracker},
        character_sheet::sheet::{Character, CharacterList},
    },
    services::{self, characters_query, CharactersQuery, GraphQLResponse},
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

    if characters.data.is_none() {
        return html! { <></> };
    }

    let characters: HashMap<i64, InitiativeInfo> = characters
        .data
        .unwrap_or_default()
        .iter()
        .map(|character| (character.id, InitiativeInfo {
            id: character.id,
            name: character.name.clone(),
            initiative: 1,
        }))
        .collect();

    html! {
        <>
            <h1>
                { format!("Campaign number {}!", props.id) }
            </h1>
            <InitiativeTracker {characters} add={Callback::from(|_| ())}/>
        </>
    }
}
