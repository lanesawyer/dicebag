use graphql_client::GraphQLQuery;
use serde_json::json;
use yew::{
    format::Json,
    html,
    services::{
        fetch::{FetchTask, Response},
        ConsoleService, FetchService,
    },
    Component, ComponentLink, Html, ShouldRender,
};
use yew_router::components::RouterAnchor;

use crate::{
    components::{Button, TextField},
    pages::character_sheet::mocks::build_bob,
    pages::character_sheet::sheet::{Character, CharacterList},
    services::{
        self, characters_query, delete_character_mutation, new_character_mutation, CharactersQuery,
        DeleteCharacterMutation, GraphQLResponse, NewCharacterMutation,
    },
    Route,
};

#[derive(Debug)]
pub enum Msg {
    ReceiveResponse(Result<GraphQLResponse<CharacterList>, anyhow::Error>),
    ReceiveNewCharacterResponse(Result<GraphQLResponse<bool>, anyhow::Error>),
    UpdateName(String),
    UpdateRace(String),
    UpdateClass(String),
    Add,
    Delete(i64),
}

#[derive(Debug)]
pub struct CharactersPage {
    pub characters: Option<Vec<Character>>,
    fetch_task: Option<FetchTask>,
    link: ComponentLink<Self>,
    error: Option<String>,
    new_name: String,
    new_race: String,
    new_class: String,
}

impl Component for CharactersPage {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let variables = characters_query::Variables {};
        let request_body = CharactersQuery::build_query(variables);
        let request_json = &json!(request_body);

        let request = services::build_request(request_json);

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
            new_name: "".to_string(),
            new_race: "".to_string(),
            new_class: "".to_string(),
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
            Msg::UpdateName(new_value) => self.new_name = new_value,
            Msg::UpdateRace(new_value) => self.new_race = new_value,
            Msg::UpdateClass(new_value) => self.new_class = new_value,
            Msg::Add => {
                let variables = new_character_mutation::Variables {
                    new_character: new_character_mutation::NewCharacter {
                        name: self.new_name.clone(),
                        class: self.new_class.clone(),
                        image: Some("".to_string()),
                        race: self.new_race.clone(),
                    },
                };
                let request_body = NewCharacterMutation::build_query(variables);
                let request_json = &json!(request_body);

                let request = services::build_request(request_json);

                let callback = self.link.callback(
                    |response: Response<Json<Result<GraphQLResponse<bool>, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::ReceiveNewCharacterResponse(data)
                    },
                );

                let task = FetchService::fetch(request, callback).expect("failed to start request");

                self.fetch_task = Some(task);
            }
            Msg::ReceiveNewCharacterResponse(result) => {
                let variables = characters_query::Variables {};
                let request_body = CharactersQuery::build_query(variables);
                let request_json = &json!(request_body);

                let request = services::build_request(request_json);

                let callback = self.link.callback(
                    |response: Response<
                        Json<Result<GraphQLResponse<CharacterList>, anyhow::Error>>,
                    >| {
                        let Json(data) = response.into_body();
                        Msg::ReceiveResponse(data)
                    },
                );

                let task = FetchService::fetch(request, callback).expect("failed to start request");
            }
            Msg::Delete(delete_id) => {
                let variables = delete_character_mutation::Variables { delete_id };
                let request_body = DeleteCharacterMutation::build_query(variables);
                let request_json = &json!(request_body);

                let request = services::build_request(request_json);

                let callback = self.link.callback(
                    |response: Response<Json<Result<GraphQLResponse<bool>, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::ReceiveNewCharacterResponse(data)
                    },
                );

                let task = FetchService::fetch(request, callback).expect("failed to start request");

                self.fetch_task = Some(task);
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
                            characters.iter().map(|c| {
                                let id = c.id;
                                html! {
                                    <>
                                        <RouterAnchor<Route> route=Route::CharacterSheet(c.id)>
                                            <div class="character-panel">
                                                <img class="character-image" src=c.image.clone()/>
                                                <span class="character-name">{c.name.clone()}</span>
                                                <span class="character-class">{c.class.clone()}</span>
                                                <span class="character-level">{c.level}</span>
                                                
                                            </div>
                                        </RouterAnchor<Route>>
                                        <Button label="Delete" on_click=self.link.callback(move |_| Msg::Delete(id)) />
                                    </>
                                }
                            }).collect::<Html>()
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
                            { self.view_input() }
                        </div>
                    </div>
                </div>
            </section>
        }
    }
}

impl CharactersPage {
    fn view_input(&self) -> Html {
        html! {
            <>
                <TextField label="Name" value=self.new_name.clone() on_change=self.link.callback(Msg::UpdateName) />
                <TextField label="Race" value=self.new_race.clone() on_change=self.link.callback(Msg::UpdateRace) />
                <TextField label="Class" value=self.new_class.clone() on_change=self.link.callback(Msg::UpdateClass) />
                <Button label="Create" on_click=self.link.callback(|_| Msg::Add) />
            </>
        }
    }

    // fn view_characters(&self) -> Html {
    //     html! {
    //         <RouterAnchor<Route> route=Route::CharacterSheet(character.id)>
    //             <div class="character-panel">
    //                 <img class="character-image" src=character.image.clone()/>
    //                 <span class="character-name">{character.name.clone()}</span>
    //                 <span class="character-class">{character.class.clone()}</span>
    //                 <span class="character-level">{character.level}</span>
    //                 <Button label="Delete" on_click=self.link.callback(|_| Msg::Delete(character.id)) />
    //             </div>
    //         </RouterAnchor<Route>>
    //     }
    // }
}
