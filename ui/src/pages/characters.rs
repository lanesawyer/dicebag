use graphql_client::GraphQLQuery;
use serde_json::json;
use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, use_state, Callback, Html};
use yew_router::components::Link;

use crate::{
    components::{Button, TextField},
    pages::character_sheet::sheet::{Character, CharacterList},
    services::{
        self, characters_query::Variables, new_character_mutation, use_query, CharactersQuery,
        GraphQLResponse, NewCharacterMutation,
    },
    AppRoute,
};

#[function_component(CharactersPage)]
pub fn characters_page() -> Html {
    let query = use_query::<CharactersQuery, CharacterList>(Variables {});

    // TODO: Better form management
    let new_name = use_state(|| "".to_string());
    let new_race = use_state(|| "".to_string());
    let new_class = use_state(|| "".to_string());
    let new_image = use_state(|| "".to_string());

    let on_change_name = {
        let new_name = new_name.clone();
        Callback::from(move |input| new_name.set(input))
    };

    let on_change_race = {
        let new_race = new_race.clone();
        Callback::from(move |input| new_race.set(input))
    };

    let on_change_class = {
        let new_class = new_class.clone();
        Callback::from(move |input| new_class.set(input))
    };

    let on_change_image = {
        let new_image = new_image.clone();
        Callback::from(move |input| new_image.set(input))
    };

    // TODO: Refresh the data to display the new character
    let on_submit = {
        let submit_name = new_name.clone();
        let submit_race = new_race.clone();
        let submit_class = new_class.clone();
        let submit_image = new_image.clone();

        Callback::from(move |_| {
            let submit_name = submit_name.clone();
            let submit_race = submit_race.clone();
            let submit_class = submit_class.clone();
            let submit_image = submit_image.clone();

            spawn_local(async move {
                let submit_name = (*submit_name).clone();
                let submit_race = (*submit_race).clone();
                let submit_class = (*submit_class).clone();
                let submit_image = (*submit_image).clone();

                let variables = new_character_mutation::Variables {
                    new_character: new_character_mutation::NewCharacter {
                        name: submit_name,
                        race: submit_race,
                        class: submit_class,
                        image: Some(submit_image),
                    },
                };
                let request_body = NewCharacterMutation::build_query(variables);
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
            {
                if let Some(character_list) = &query.data {
                    character_list.characters.iter().map(view_characters).collect::<Html>()
                } else {
                    // TODO: Character skeleton while loading
                    html! { <></> }
                }
            }
            <div class="list-item add-character-panel">
                <TextField label="Name" value={(*new_name).clone()} on_change={on_change_name} />
                <TextField label="Race" value={(*new_race).clone()} on_change={on_change_race} />
                <TextField label="Class" value={(*new_class).clone()} on_change={on_change_class} />
                <TextField label="Image" value={(*new_image).clone()} on_change={on_change_image} />
                <Button label="Create" icon_name={"plus".to_string()} on_click={on_submit} />
            </div>
        </section>
    }
}

fn view_characters(character: &Character) -> Html {
    html! {
        <div class="list-item">
            <Link<AppRoute> to={AppRoute::CharacterSheet { id: character.id }}>
                <div class="character-panel">
                    <img class="character-image" src={character.image.clone()}/>
                    <span class="character-name">{character.name.clone()}</span>
                    <span class="character-class">{character.class.clone()}</span>
                    <span class="character-level">{character.level}</span>
                </div>
            </Link<AppRoute>>
        </div>
    }
}
