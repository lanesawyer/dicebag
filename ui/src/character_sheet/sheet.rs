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
use crate::dice_tower::tower::Tower;
use graphql_client::GraphQLQuery;
use serde::Deserialize;
use serde_json::json;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{format::Json, services::ConsoleService};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

#[derive(GraphQLQuery)]
#[graphql(schema_path = "src/schema.json", query_path = "src/queries.graphql")]
struct CharacterQuery;

#[derive(Debug)]
pub enum Msg {
    ReceiveResponse(Result<GraphQLResponse<CharacterList>, anyhow::Error>),
}

#[derive(Debug)]
pub struct CharacterSheet {
    pub character: Option<Character>,
    fetch_task: Option<FetchTask>,
    link: ComponentLink<Self>,
    error: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Character {
    // Info
    pub name: String,
    pub class: String, // TODO: enum
    pub level: usize,
    pub background: String,
    pub race: String,      // TODO: enum?
    pub alignment: String, // TODO: enum
    pub experience_points: usize,

    // Stats
    pub strength: usize,
    pub dexterity: usize,
    pub constitution: usize,
    pub intelligence: usize,
    pub wisdom: usize,
    pub charisma: usize,

    // Other
    pub proficiency_bonus: usize,
    pub has_inspiration: bool,
    pub personality_traits: String,
    pub ideals: String,
    pub bonds: String,
    pub flaws: String,
    pub features_and_traits: String,
    pub other_proficiencies_and_languages: String,
    pub armor_class: usize,
    pub speed: usize,
    pub hit_points: usize,
    pub current_hit_points: usize,
    pub temporary_hit_points: usize,
    pub hit_dice: usize,
    pub used_hit_dice: usize,
    pub saves: usize,
    pub failures: usize,

    pub equipment: String,
    pub copper: usize,
    pub silver: usize,
    pub electrum: usize,
    pub platinum: usize,
    pub gold: usize,
}

#[derive(Debug, Deserialize)]
pub struct GraphQLResponse<T> {
    pub data: T,
}

#[derive(Debug, Deserialize)]
pub struct CharacterList {
    pub character: Vec<Character>,
}

impl Component for CharacterSheet {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let variables = character_query::Variables {};
        let request_body = CharacterQuery::build_query(variables);
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
                        self.character = Some(character.data.character.into_iter().nth(1).unwrap());
                        ConsoleService::log(&format!("win {:?}", self.character));
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
        ConsoleService::log(&format!("render time! {:?}", self.character));
        let character = self.character.as_ref().unwrap();
        let skills = build_skills(character);
        let saving_throws = build_saving_throws(character);
        html! {
            <>
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
            </>
        }
    }
}
