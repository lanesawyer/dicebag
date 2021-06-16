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
    stat_block::StatBlock,
    text_block::TextBlock,
};
use crate::services::characters_query;
use crate::{
    dice_tower::tower::Tower,
    services::{CharactersQuery, GraphQLResponse},
};
use graphql_client::GraphQLQuery;
use serde::Deserialize;
use serde_json::json;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{format::Json, services::ConsoleService};
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Debug)]
pub enum Msg {
    ReceiveResponse(Result<GraphQLResponse<CharacterList>, anyhow::Error>),
}

#[derive(Properties, Clone, Debug)]
pub struct CharacterSheetProps {
    pub id: String,
}

#[derive(Debug)]
pub struct CharacterSheetPage {
    pub props: CharacterSheetProps,
    pub character: Option<Character>,
    fetch_task: Option<FetchTask>,
    link: ComponentLink<Self>,
    error: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Character {
    pub id: String,
    pub image: String,

    // Info
    pub name: String,
    pub class: String, // TODO: enum
    pub level: i64,
    pub background: String,
    pub race: String,      // TODO: enum?
    pub alignment: String, // TODO: enum
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
    pub personality_traits: String,
    pub ideals: String,
    pub bonds: String,
    pub flaws: String,
    pub features_and_traits: String,
    pub other_proficiencies_and_languages: String,
    pub armor_class: i64,
    pub speed: i64,
    pub hit_points: i64,
    pub current_hit_points: i64,
    pub temporary_hit_points: i64,
    pub hit_dice: i64,
    pub used_hit_dice: i64,
    pub saves: i64,
    pub failures: i64,

    pub equipment: String,
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

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let variables = characters_query::Variables {};
        let request_body = CharactersQuery::build_query(variables);
        let request_json = &json!(request_body);

        ConsoleService::log(&format!("{:?}", &request_json));

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
            props,
            character: Some(build_bob()),
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
                        self.character = Some(
                            character
                                .data
                                .characters
                                .into_iter()
                                .find(|character| character.id == self.props.id)
                                .unwrap(),
                        );
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
        let character = self.character.as_ref().unwrap();
        let skills = build_skills(character);
        let saving_throws = build_saving_throws(character);
        html! {
            <section id="character-sheet">
                <CharacterInfo
                    name={character.name.clone()}
                    class={character.class.clone()}
                    level={character.level}
                    background={character.background.clone()}
                    race={character.race.clone()}
                    alignment={character.alignment.clone()}
                    experience_points={character.experience_points}
                />
                <section id="stat-block" class="stats">
                    <StatBlock name="Strength" value={character.strength} />
                    <StatBlock name="Dexterity" value={character.dexterity} />
                    <StatBlock name="Constitution" value={character.constitution} />
                    <StatBlock name="Intelligence" value={character.intelligence} />
                    <StatBlock name="Wisdom" value={character.wisdom} />
                    <StatBlock name="Charisma" value={character.charisma} />
                </section>
                <Inspiration value=character.has_inspiration />
                <ProficiencyBonus value=character.proficiency_bonus />
                <SavingThrows items=saving_throws />
                <Skills items=skills />
                <PassivePerception value=1 />
                <section id="other-proficiencies-and-languages">
                    <TextBlock name="Other Proficiencies & Languages" value=character.other_proficiencies_and_languages.clone() />
                </section>
                <ArmorClass value=character.armor_class />
                <Initiative value=character.dexterity />
                <Speed value=character.speed />
                <HitPoints
                    maximum=character.hit_points
                    current=character.current_hit_points
                    temporary=character.temporary_hit_points
                />
                <HitDice total=character.hit_dice used=character.used_hit_dice />
                <DeathSavingThrows saves=character.saves failures=character.failures />
                <Attacks attacks=Vec::new() />
                <Money
                    copper=character.copper
                    silver=character.silver
                    electrum=character.electrum
                    gold=character.gold
                    platinum=character.platinum
                />
                <section id="equipment">
                    <TextBlock name="Equipment" value=character.equipment.clone() />
                </section>
                <section id="character-details">
                    <TextBlock name="Personality Traits" value=character.personality_traits.clone() />
                    <TextBlock name="Ideals" value=character.ideals.clone() />
                    <TextBlock name="Bonds" value=character.bonds.clone() />
                    <TextBlock name="Flaws" value=character.flaws.clone() />
                </section>
                <section id="features-traits">
                    <TextBlock name="Features & Traits" value=character.features_and_traits.clone() />
                </section>
                <Tower />
            </section>
        }
    }
}