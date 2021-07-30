use graphql_client::GraphQLQuery;
use serde::Deserialize;
use serde_json::json;
use yew::services::fetch::{FetchService, FetchTask, Response};
use yew::{format::Json, services::ConsoleService};
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_router::{
    agent::RouteRequest,
    prelude::{Route, RouteAgentDispatcher},
};

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
    stat_block::{StatBlock, Stat},
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
    ReceiveResponse(Result<GraphQLResponse<CharacterList>, anyhow::Error>),
    Delete(i64),
    RedirectMsg,
}

#[derive(Properties, Clone, Debug)]
pub struct CharacterSheetProps {
    pub id: i64,
}

#[derive(Debug)]
pub struct CharacterSheetPage {
    pub props: CharacterSheetProps,
    pub character: Option<Character>,
    fetch_task: Option<FetchTask>,
    link: ComponentLink<Self>,
    error: Option<String>,
    route_agent: RouteAgentDispatcher<()>,
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

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let route_agent = RouteAgentDispatcher::new();

        let variables = characters_query::Variables {};
        let query = CharactersQuery::build_query(variables);
        let request_json = &json!(query);

        ConsoleService::log(&format!("{:?}", &request_json));

        let request = services::build_request(request_json);

        let callback = link.callback(
            |response: Response<Json<Result<GraphQLResponse<CharacterList>, anyhow::Error>>>| {
                let Json(data) = response.into_body();
                Msg::ReceiveResponse(data)
            },
        );

        let task = FetchService::fetch(request, callback).expect("failed to start request");

        // let task = services::post(query,
        //     link,
        //     Box::new(|data| Msg::ReceiveResponse(data))
        // );

        Self {
            props,
            character: Some(build_bob()),
            link,
            fetch_task: Some(task),
            error: None,
            route_agent,
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
            Msg::Delete(delete_id) => {
                let variables = delete_character_mutation::Variables { delete_id };
                let request_body = DeleteCharacterMutation::build_query(variables);
                let request_json = &json!(request_body);

                let request = services::build_request(request_json);

                let callback = self.link.callback(
                    |response: Response<Json<Result<GraphQLResponse<bool>, anyhow::Error>>>| {
                        // TODO: Error pop up if delete fails
                        let Json(_data) = response.into_body();
                        Msg::RedirectMsg
                    },
                );

                let task = FetchService::fetch(request, callback).expect("failed to start request");

                self.fetch_task = Some(task);
            }
            Msg::RedirectMsg => {
                let route = Route::from(AppRoute::Characters);
                self.route_agent.send(RouteRequest::ChangeRoute(route));
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let character = self.character.as_ref().unwrap();
        let delete_id = character.id;
        let skills = build_skills(character);
        let saving_throws = build_saving_throws(character);
        html! {
            <section id="character-sheet">
                <CharacterInfo
                    image=character.image.clone()
                    name=character.name.clone()
                    class=character.class.clone()
                    level=character.level
                    background=character.background.clone()
                    race=character.race.clone()
                    alignment=character.alignment.clone()
                    experience_points=character.experience_points
                />
                <section id="settings">
                    <Icon name="cog" />
                    <Button label="Delete" button_type=ButtonType::Danger on_click=self.link.callback(move |_| Msg::Delete(delete_id)) />
                </section>
                <section id="stat-blocks" class="stats">
                    <StatBlock stat=Stat::Strength name="Strength" value=character.strength />
                    <StatBlock stat=Stat::Dexterity name="Dexterity" value=character.dexterity />
                    <StatBlock stat=Stat::Constitution name="Constitution" value=character.constitution />
                    <StatBlock stat=Stat::Intelligence name="Intelligence" value=character.intelligence />
                    <StatBlock stat=Stat::Wisdom name="Wisdom" value=character.wisdom />
                    <StatBlock stat=Stat::Charisma name="Charisma" value=character.charisma />
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
            </section>
        }
    }
}
