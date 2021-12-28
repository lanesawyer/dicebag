use gloo_console::log;
use graphql_client::GraphQLQuery;
use serde::Deserialize;
use serde_json::json;
use yew::{html, Component, Context, Html, Properties};
use yew_router::prelude::*;

use super::{
    armor_class::ArmorClass,
    attacks::Attacks,
    character_info::CharacterInfo,
    death_saving_throws::DeathSavingThrows,
    hit_dice::HitDice,
    hit_points::HitPoints,
    initiative::Initiative,
    inspiration::Inspiration,
    mocks::{build_bob, build_saving_throws, build_skills},
    money::Money,
    passive_perception::PassivePerception,
    proficiency_bonus::ProficiencyBonus,
    saving_throws::SavingThrows,
    skills::Skills,
    speed::Speed,
    stat_block::{Stat, StatBlock},
    text_block::TextBlock,
};
use crate::{
    components::{Button, ButtonType, Icon},
    services::{self, characters_query, delete_character_mutation, DeleteCharacterMutation},
    services::{CharactersQuery, GraphQLResponse},
    AppRoute,
};

#[derive(Debug)]
pub enum Msg {
    ReceiveResponse(Result<GraphQLResponse<CharacterList>, reqwest::Error>),
    Delete(i64),
    Redirect,
    Error,
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct CharacterSheetProps {
    pub id: i64,
}

#[derive(Debug)]
pub struct CharacterSheetPage {
    pub character: Option<Character>,
    error: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Character {
    pub id: i64,
    pub image: Option<String>,

    // Info
    pub name: String,
    pub class: String, // TODO: enum
    pub level: i64,
    pub background: Option<String>,
    pub race: String,              // TODO: enum?
    pub alignment: Option<String>, // TODO: enum
    pub experience_points: i64,

    // Stats
    pub strength: i64,
    pub dexterity: i64,
    pub constitution: i64,
    pub intelligence: i64,
    pub wisdom: i64,
    pub charisma: i64,

    // Other
    pub proficiency_bonus: i64,
    pub has_inspiration: bool,

    pub personality_traits: Option<String>,
    pub ideals: Option<String>,
    pub bonds: Option<String>,
    pub flaws: Option<String>,
    pub features_and_traits: Option<String>,
    pub other_proficiencies_and_languages: Option<String>,

    pub armor_class: i64,
    pub speed: i64,
    pub hit_points: i64,
    pub current_hit_points: i64,
    pub temporary_hit_points: i64,
    pub hit_dice: i64,
    pub used_hit_dice: i64,
    pub saves: i64,
    pub failures: i64,
    pub equipment: Option<String>,
    pub copper: i64,
    pub silver: i64,
    pub electrum: i64,
    pub platinum: i64,
    pub gold: i64,
}

#[derive(Debug, Deserialize)]
pub struct CharacterList {
    pub characters: Vec<Character>,
}

impl Component for CharacterSheetPage {
    type Message = Msg;
    type Properties = CharacterSheetProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async {
            let variables = characters_query::Variables {};
            let query = CharactersQuery::build_query(variables);

            let request_json = &json!(query);
            log!(&format!("{:?}", &request_json));

            let request = services::build_request(request_json).await;
            if let Ok(response) = request {
                let json = response.json::<GraphQLResponse<CharacterList>>().await;
                Msg::ReceiveResponse(json)
            } else {
                Msg::Error
            }
        });

        Self {
            character: Some(build_bob()),
            error: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ReceiveResponse(response) => match response {
                Ok(character) => {
                    self.character = Some(
                        character
                            .data
                            .characters
                            .into_iter()
                            .find(|character| character.id == ctx.props().id)
                            .unwrap(),
                    );
                }
                Err(error) => {
                    self.error = Some(error.to_string());
                    log!(&format!("error {:?}", error));
                }
            },
            Msg::Delete(delete_id) => {
                let variables = delete_character_mutation::Variables { delete_id };
                let request_body = DeleteCharacterMutation::build_query(variables);

                ctx.link().send_future(async move {
                    let request_json = &json!(request_body);
                    let request = services::build_request(request_json).await;
                    if let Ok(response) = request {
                        let _json = response.json::<GraphQLResponse<bool>>().await;
                        Msg::Redirect
                    } else {
                        Msg::Error
                    }
                });
            }
            Msg::Redirect => {
                let history = ctx.link().history().unwrap();
                history.push(AppRoute::Characters);
            }
            Msg::Error => {}
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let character = self.character.as_ref().unwrap();
        let delete_id = character.id;
        let skills = build_skills(character);
        let saving_throws = build_saving_throws(character);
        html! {
            <section id="character-sheet">
                <CharacterInfo
                    image={character.image.clone()}
                    name={character.name.clone()}
                    class={character.class.clone()}
                    level={character.level}
                    background={character.background.clone()}
                    race={character.race.clone()}
                    alignment={character.alignment.clone()}
                    experience_points={character.experience_points}
                />
                <section id="settings">
                    <Icon name="cog" />
                    <Button label="Delete" icon_name={"trash".to_string()} button_type={ButtonType::Danger} on_click={ctx.link().callback(move |_| Msg::Delete(delete_id))} />
                </section>
                <section id="stat-blocks" class="stats">
                    <StatBlock stat={Stat::Strength} name="Strength" value={character.strength} />
                    <StatBlock stat={Stat::Dexterity} name="Dexterity" value={character.dexterity} />
                    <StatBlock stat={Stat::Constitution} name="Constitution" value={character.constitution} />
                    <StatBlock stat={Stat::Intelligence} name="Intelligence" value={character.intelligence} />
                    <StatBlock stat={Stat::Wisdom} name="Wisdom" value={character.wisdom} />
                    <StatBlock stat={Stat::Charisma} name="Charisma" value={character.charisma} />
                </section>
                <Inspiration value={character.has_inspiration} />
                <ProficiencyBonus value={character.proficiency_bonus} />
                <SavingThrows items={saving_throws} />
                <Skills items={skills} />
                <PassivePerception value={1} />
                <section id="other-proficiencies-and-languages">
                    <TextBlock name="Other Proficiencies & Languages" value={character.other_proficiencies_and_languages.clone()} />
                </section>
                <ArmorClass value={character.armor_class} />
                <Initiative value={character.dexterity} />
                <Speed value={character.speed} />
                <HitPoints
                    maximum={character.hit_points}
                    current={character.current_hit_points}
                    temporary={character.temporary_hit_points}
                />
                <HitDice total={character.hit_dice} used={character.used_hit_dice} />
                <DeathSavingThrows saves={character.saves} failures={character.failures} />
                <Attacks attacks={Vec::new()} />
                <Money
                    copper={character.copper}
                    silver={character.silver}
                    electrum={character.electrum}
                    gold={character.gold}
                    platinum={character.platinum}
                />
                <section id="equipment">
                    <TextBlock name="Equipment" value={character.equipment.clone()} />
                </section>
                <section id="character-details">
                    <TextBlock name="Personality Traits" value={character.personality_traits.clone()} />
                    <TextBlock name="Ideals" value={character.ideals.clone()} />
                    <TextBlock name="Bonds" value={character.bonds.clone()} />
                    <TextBlock name="Flaws" value={character.flaws.clone()} />
                </section>
                <section id="features-traits">
                    <TextBlock name="Features & Traits" value={character.features_and_traits.clone()} />
                </section>
            </section>
        }
    }
}
