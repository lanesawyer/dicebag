use graphql_client::GraphQLQuery;
use serde_json::json;
use yew::{
    format::Json,
    html,
    services::{
        fetch::{FetchTask, Request, Response},
        ConsoleService, FetchService,
    },
    Component, ComponentLink, Html, ShouldRender,
};
use yew_router::components::RouterAnchor;

use crate::{
    pages::character_sheet::mocks::build_bob,
    pages::character_sheet::sheet::{Character, CharacterList},
    services::{characters_query, CharactersQuery, GraphQLResponse},
    Route,
};

#[derive(Debug)]
pub enum Msg {
    ReceiveResponse(Result<GraphQLResponse<CharacterList>, anyhow::Error>),
}

#[derive(Debug)]
pub struct CharactersPage {
    pub characters: Option<Vec<Character>>,
    fetch_task: Option<FetchTask>,
    link: ComponentLink<Self>,
    error: Option<String>,
}

impl Component for CharactersPage {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let variables = characters_query::Variables {};
        let request_body = CharactersQuery::build_query(variables);
        let request_json = &json!(request_body);

        // TODO: Pull URL from .env
        let request = Request::post("http://127.0.0.1:8000/graphql")
            .header("Content-Type", "application/json")
            .body(Json(request_json))
            .expect("Could not build that request.");

        let callback = link.callback(
            |response: Response<Json<Result<GraphQLResponse<CharacterList>, anyhow::Error>>>| {
                let Json(data) = response.into_body();
                Msg::ReceiveResponse(data)
            },
        );

        let task = FetchService::fetch(request, callback).expect("failed to start request");

        Self {
            characters: Some(vec![build_bob()]),
            link,
            fetch_task: Some(task),
            error: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ReceiveResponse(response) => {
                match response {
                    Ok(character) => {
                        self.characters = Some(character.data.characters);
                    }
                    Err(error) => {
                        self.error = Some(error.to_string());
                        ConsoleService::log(&format!("error {:?}", error));
                    }
                }
                self.fetch_task = None;
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <section id="characters-page">
                <div id="characters">
                    {
                        if let Some(characters) = &self.characters {
                            characters.iter().map(view_characters).collect::<Html>()
                        } else {
                            // TODO: Character skeleton
                            html! { <></> }
                        }
                    }
                    // TODO: Click action to add new character
                    <div class="add-character-panel">
                        <div>
                            <div>{ "➕" }</div>
                            <div>{ "Create" }</div>
                        </div>
                    </div>
                </div>
            </section>
        }
    }
}

fn view_characters(character: &Character) -> Html {
    html! {
        <RouterAnchor<Route> route=Route::CharacterSheet(character.id)>
            <div class="character-panel">
                <img class="character-image" src=character.image.clone()/>
                <span class="character-name">{character.name.clone()}</span>
                <span class="character-class">{character.class.clone()}</span>
                <span class="character-level">{character.level}</span>
            </div>
        </RouterAnchor<Route>>
    }
}
