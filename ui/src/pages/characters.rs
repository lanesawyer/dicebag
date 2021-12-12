use gloo_console::log;
use graphql_client::GraphQLQuery;
use serde_json::json;
use yew::{
    html,
    Component, Html, Context,
};
use yew_router::components::Link;

use crate::{
    components::{Button, TextField},
    pages::character_sheet::mocks::build_bob,
    pages::character_sheet::sheet::{Character, CharacterList},
    services::{
        self, characters_query, new_character_mutation, CharactersQuery, GraphQLResponse,
        NewCharacterMutation,
    },
    AppRoute,
};

#[derive(Debug)]
pub enum Msg {
    ReceiveResponse(Result<GraphQLResponse<CharacterList>, reqwest::Error>),
    ReceiveNewCharacterResponse(Result<GraphQLResponse<bool>, reqwest::Error>),
    UpdateName(String),
    UpdateRace(String),
    UpdateClass(String),
    UpdateImage(String),
    Add,
    Error,
}

#[derive(Debug)]
pub struct CharactersPage {
    pub characters: Option<Vec<Character>>,
    // fetch_task: Option<FetchTask>,
    error: Option<String>,
    new_name: String,
    new_race: String,
    new_class: String,
    new_image: String,
}

impl Component for CharactersPage {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {

        ctx.link().send_future(async move {
            let variables = characters_query::Variables {};
            let request_body = CharactersQuery::build_query(variables);
            let request_json = &json!(request_body);
            
            let request = services::build_request(request_json).await;
            if let Ok(response) = request {
                let json = response.json::<GraphQLResponse<CharacterList>>().await;
                Msg::ReceiveResponse(json)
            } else {
                Msg::Error
            }
        });

        // let request = services::build_request(request_json);
        // ctx.link().send_future(request);
        // if let Ok(response) = request {
        //     let json = response.json::<GraphQLResponse<CharacterList>>();
        //     ctx.link().send_message(Msg::ReceiveResponse(json));
        // }
        // let callback = ctx.link().callback(
        //     |response: Response<Json<Result<GraphQLResponse<CharacterList>, anyhow::Error>>>| {
        //         let Json(data) = response.into_body();
        //         Msg::ReceiveResponse(data)
        //     },
        // );
        // callback.call();

        // let task = FetchService::fetch(request, callback).expect("failed to start request");

        Self {
            characters: Some(vec![build_bob()]),
            // fetch_task: Some(task),
            error: None,
            new_name: "".to_string(),
            new_race: "".to_string(),
            new_class: "".to_string(),
            new_image: "".to_string(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ReceiveResponse(response) => {
                match response {
                    Ok(character) => {
                        self.characters = Some(character.data.characters);
                    }
                    Err(error) => {
                        self.error = Some(error.to_string());
                        log!(&format!("error {:?}", error));
                    }
                }
                // self.fetch_task = None;
            }
            Msg::UpdateName(new_value) => self.new_name = new_value,
            Msg::UpdateRace(new_value) => self.new_race = new_value,
            Msg::UpdateClass(new_value) => self.new_class = new_value,
            Msg::UpdateImage(new_value) => self.new_image = new_value,
            Msg::Add => {
                let name = self.new_name.clone();
                let class = self.new_class.clone();
                let image = Some(self.new_image.to_string());
                let race = self.new_race.clone();
                ctx.link().send_future(async move {
                    let variables = new_character_mutation::Variables {
                        new_character: new_character_mutation::NewCharacter {
                            name,
                            class,
                            image,
                            race,
                        },
                    };
                    let request_body = NewCharacterMutation::build_query(variables);
                    let request_json = &json!(request_body);
                    let request = services::build_request(request_json).await;
                    if let Ok(response) = request {
                        let json = response.json::<GraphQLResponse<bool>>().await;
                        Msg::ReceiveNewCharacterResponse(json)
                    } else {
                        Msg::Error
                    }
                });

                // let callback = ctx.link().callback(
                //     |response: Response<Json<Result<GraphQLResponse<bool>, reqwest::Error>>>| {
                //         let Json(data) = response.into_body();
                //         Msg::ReceiveNewCharacterResponse(data)
                //     },
                // );

                // let task = FetchService::fetch(request, callback).expect("failed to start request");
                // self.fetch_task = Some(task);
            }
            Msg::ReceiveNewCharacterResponse(_result) => {
                // TODO: Error popup if new character fails

                ctx.link().send_future(async {
                    let variables = characters_query::Variables {};
                    let request_body = CharactersQuery::build_query(variables);
                    let request_json = &json!(request_body);
                    let request = services::build_request(request_json).await;
                    if let Ok(response) = request {
                        let json = response.json::<GraphQLResponse<CharacterList>>().await;
                        Msg::ReceiveResponse(json)
                    } else {
                        Msg::Error
                    }
                });

                // let request = services::build_request(request_json);

                // if let Ok(response) = request {
                //     let json = response.json::<GraphQLResponse<CharacterList>>();
                //     ctx.link().send_message(Msg::ReceiveResponse(json));
                // }

                // let callback = ctx.link().callback(
                //     |response: Response<
                //         Json<Result<GraphQLResponse<CharacterList>, anyhow::Error>>,
                //     >| {
                //         let Json(data) = response.into_body();
                //         Msg::ReceiveResponse(data)
                //     },
                // );

                // let task = FetchService::fetch(request, callback).expect("failed to start request");
                // self.fetch_task = Some(task);
            }
            Msg::Error => {
                ()
            }
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <section id="characters-page">
                <div id="characters">
                    {
                        if let Some(characters) = &self.characters {
                            characters.iter().map(|c| self.view_characters(c)).collect::<Html>()
                        } else {
                            // TODO: Character skeleton while loading
                            html! { <></> }
                        }
                    }
                    <div class="add-character-panel">
                        { self.view_input(ctx) }
                    </div>
                </div>
            </section>
        }
    }
}

impl CharactersPage {
    fn view_input(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <TextField label="Name" value={self.new_name.clone()} on_change={ctx.link().callback(Msg::UpdateName)} />
                <TextField label="Race" value={self.new_race.clone()} on_change={ctx.link().callback(Msg::UpdateRace)} />
                <TextField label="Class" value={self.new_class.clone()} on_change={ctx.link().callback(Msg::UpdateClass)} />
                <TextField label="Image" value={self.new_image.clone()} on_change={ctx.link().callback(Msg::UpdateImage)} />
                <Button label="Create" icon_name={"plus".to_string()} on_click={ctx.link().callback(|_| Msg::Add)} />
            </>
        }
    }

    fn view_characters(&self, character: &Character) -> Html {
        html! {
            <Link<AppRoute> to={AppRoute::CharacterSheet { id: character.id }}>
                <div class="character-panel">
                    <img class="character-image" src={character.image.clone()}/>
                    <span class="character-name">{character.name.clone()}</span>
                    <span class="character-class">{character.class.clone()}</span>
                    <span class="character-level">{character.level}</span>
                </div>
            </Link<AppRoute>>
        }
    }
}
