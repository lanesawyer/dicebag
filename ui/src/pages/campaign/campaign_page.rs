use std::collections::HashMap;

use graphql_client::GraphQLQuery;
use serde_json::json;
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, Callback, Properties};
use yew_router::history::History;
use yew_router::hooks::use_history;

use crate::{
    components::{Button, ButtonType, Icon},
    navigation::AppRoute,
    pages::{
        campaign::initiative_tracker::{InitiativeInfo, InitiativeTracker},
        character_sheet::sheet::CharacterList,
    },
    services::{
        self, characters_query, delete_campaign_mutation, use_query, CharactersQuery,
        DeleteCampaignMutation, GraphQLResponse,
    },
};

#[derive(Properties, Clone, PartialEq)]
pub struct CampaignProps {
    pub id: i64,
}

#[function_component(CampaignPage)]
pub fn campaign_page(props: &CampaignProps) -> Html {
    let characters = use_query::<CharactersQuery, CharacterList>(characters_query::Variables {});
    let history = use_history().expect("history should be available");

    if characters.data.is_none() {
        return html! { <></> };
    }

    let delete_campaign = {
        let delete_id = props.id;
        Callback::from(move |_| {
            let delete_history = history.clone();
            spawn_local(async move {
                let variables = delete_campaign_mutation::Variables { delete_id };
                let request_body = DeleteCampaignMutation::build_query(variables);
                let request_json = &json!(request_body);
                let request = services::build_request(request_json).await;

                if let Ok(response) = request {
                    let json = response
                        .json::<GraphQLResponse<delete_campaign_mutation::ResponseData>>()
                        .await;
                    match json {
                        Ok(_response) => delete_history.push(AppRoute::Campaigns),
                        Err(_error) => (),
                    }
                }
            });
        })
    };

    let characters_initiative: HashMap<String, InitiativeInfo> = characters
        .data
        .unwrap_or_default()
        .characters
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
            <>{characters.error.unwrap_or_else(|| "".to_string())}</>
            <section id="settings">
                <Icon name="cog" />
                <Button label="Delete" icon_name={"trash".to_string()} button_type={ButtonType::Danger} on_click={delete_campaign} />
            </section>
            <InitiativeTracker {characters_initiative} />
        </>
    }
}
