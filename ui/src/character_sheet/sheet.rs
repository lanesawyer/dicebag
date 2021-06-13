use super::{
    armor_class::ArmorClass, attacks::Attacks, character_info::CharacterInfo,
    death_saving_throws::DeathSavingThrows, hit_dice::HitDice, hit_points::HitPoints,
    initiative::Initiative, inspiration::Inspiration, money::Money,
    passive_perception::PassivePerception, proficiency_bonus::ProficiencyBonus,
    saving_throws::SavingThrows, skills::Skills, speed::Speed, stat_block::StatBlock,
    text_block::TextBlock,
};
use crate::components::Skill;
use crate::dice_tower::tower::Tower;
use graphql_client::GraphQLQuery;
use serde::Deserialize;
use serde_json::json;
use yew::format::Json;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

#[derive(GraphQLQuery)]
#[graphql(schema_path = "src/schema.json", query_path = "src/queries.graphql")]
struct CharacterQuery;

#[derive(Debug)]
pub enum Msg {
    GetCharacter,
    ReceiveResponse(Result<Character, anyhow::Error>),
}

#[derive(Debug)]
pub struct CharacterSheet {
    pub character: Option<Character>,
    fetch_task: Option<FetchTask>,
    link: ComponentLink<Self>,
    error: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Character {
    // Info
    name: String,
    class: String, // TODO: enum
    level: usize,
    background: String,
    race: String,      // TODO: enum?
    alignment: String, // TODO: enum
    experience_points: usize,

    // Stats
    strength: usize,
    dexterity: usize,
    constitution: usize,
    intelligence: usize,
    wisdom: usize,
    charisma: usize,

    // Other
    proficiency_bonus: usize,
    has_inspiration: bool,
    personality_traits: String,
    ideals: String,
    bonds: String,
    flaws: String,
    features_and_traits: String,
    other_proficiencies_and_languages: String,
    armor_class: usize,
    speed: usize,
    hit_points: usize,
    current_hit_points: usize,
    temporary_hit_points: usize,
    hit_dice: usize,
    used_hit_dice: usize,
    saves: usize,
    failures: usize,

    equipment: String,
    copper: usize,
    silver: usize,
    electrum: usize,
    platinum: usize,
    gold: usize,
}

impl Component for CharacterSheet {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.callback(|_: i32| Msg::GetCharacter);

        Self {
            character: Some(Character {
                name: "Bob the Builder".to_string(),
                class: "Artificer".to_string(),
                level: 3,
                background: "Construction worker".to_string(),
                race: "Human".to_string(),
                alignment: "Lawful Good".to_string(),
                experience_points: 100,
                strength: 12,
                dexterity: 8,
                constitution: 14,
                intelligence: 10,
                wisdom: 12,
                charisma: 8,
                proficiency_bonus: 1,
                has_inspiration: false,
                personality_traits: "Happy go lucky".to_string(),
                ideals: "A clean site is a safe site".to_string(),
                bonds: "I will always build a solid foundation".to_string(),
                flaws: "Talks to machines".to_string(),
                features_and_traits: "Organizer: Goes into battle with an extra action to plan"
                    .to_string(),
                other_proficiencies_and_languages: "English and Spanish".to_string(),
                armor_class: 13,
                hit_points: 20,
                current_hit_points: 18,
                temporary_hit_points: 0,
                hit_dice: 3,
                used_hit_dice: 0,
                saves: 2,
                failures: 1,
                speed: 30,
                equipment: "Hammer".to_string(),
                copper: 10,
                silver: 5,
                electrum: 1,
                gold: 3,
                platinum: 0,
            }),
            link,
            fetch_task: None,
            error: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GetCharacter => {
                // this is the important line
                let variables = character_query::Variables {};
                let request_body = CharacterQuery::build_query(variables);
                let request_json = &json!(request_body);

                let request = Request::post("/graphql")
                    .header("Content-Type", "application/json")
                    .body(Json(request_json))
                    .expect("Could not build that request.");

                let callback = self.link.callback(
                    |response: Response<Json<Result<Character, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::ReceiveResponse(data)
                    },
                );

                let task = FetchService::fetch(request, callback).expect("failed to start request");
                // 4. store the task so it isn't canceled immediately
                self.fetch_task = Some(task);
            }
            Msg::ReceiveResponse(response) => {
                match response {
                    Ok(character) => {
                        self.character = Some(character);
                    }
                    Err(error) => {
                        self.error = Some(error.to_string())
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
        let saving_throws = vec![
            Skill {
                has_proficiency: false,
                ability_score: character.strength,
                name: "Strength".to_string(),
                related_ability: None,
            },
            Skill {
                has_proficiency: false,
                ability_score: character.dexterity,
                name: "Dexterity".to_string(),
                related_ability: None,
            },
            Skill {
                has_proficiency: true,
                ability_score: character.constitution,
                name: "Constitution".to_string(),
                related_ability: None,
            },
            Skill {
                has_proficiency: false,
                ability_score: character.intelligence,
                name: "Intelligence".to_string(),
                related_ability: None,
            },
            Skill {
                has_proficiency: false,
                ability_score: character.charisma,
                name: "Charisma".to_string(),
                related_ability: None,
            },
        ];

        let skills = vec![
            Skill {
                has_proficiency: false,
                ability_score: character.dexterity,
                name: "Acrobatics".to_string(),
                related_ability: Some("Dex".to_string()),
            },
            Skill {
                has_proficiency: false,
                ability_score: character.wisdom,
                name: "Animal Handling".to_string(),
                related_ability: Some("Wis".to_string()),
            },
            Skill {
                has_proficiency: false,
                ability_score: character.intelligence,
                name: "Arcana".to_string(),
                related_ability: Some("Int".to_string()),
            },
            Skill {
                has_proficiency: false,
                ability_score: character.strength,
                name: "Athletics".to_string(),
                related_ability: Some("Str".to_string()),
            },
            Skill {
                has_proficiency: false,
                ability_score: character.charisma,
                name: "Deception".to_string(),
                related_ability: Some("Cha".to_string()),
            },
            Skill {
                has_proficiency: false,
                ability_score: character.intelligence,
                name: "History".to_string(),
                related_ability: Some("Int".to_string()),
            },
            Skill {
                has_proficiency: false,
                ability_score: character.wisdom,
                name: "Insight".to_string(),
                related_ability: Some("Wis".to_string()),
            },
            Skill {
                has_proficiency: false,
                ability_score: character.charisma,
                name: "Intimidation".to_string(),
                related_ability: Some("Char".to_string()),
            },
            Skill {
                has_proficiency: false,
                ability_score: character.intelligence,
                name: "Investigation".to_string(),
                related_ability: Some("Int".to_string()),
            },
            Skill {
                has_proficiency: false,
                ability_score: character.wisdom,
                name: "Medicine".to_string(),
                related_ability: Some("Wis".to_string()),
            },
            Skill {
                has_proficiency: false,
                ability_score: character.intelligence,
                name: "Nature".to_string(),
                related_ability: Some("Int".to_string()),
            },
            Skill {
                has_proficiency: false,
                ability_score: character.wisdom,
                name: "Perception".to_string(),
                related_ability: Some("Wis".to_string()),
            },
            Skill {
                has_proficiency: false,
                ability_score: character.charisma,
                name: "Performance".to_string(),
                related_ability: Some("Cha".to_string()),
            },
            Skill {
                has_proficiency: false,
                ability_score: character.charisma,
                name: "Persuasion".to_string(),
                related_ability: Some("Cha".to_string()),
            },
            Skill {
                has_proficiency: false,
                ability_score: character.intelligence,
                name: "Religion".to_string(),
                related_ability: Some("Int".to_string()),
            },
            Skill {
                has_proficiency: false,
                ability_score: character.dexterity,
                name: "Sleight of Hand".to_string(),
                related_ability: Some("Dex".to_string()),
            },
            Skill {
                has_proficiency: false,
                ability_score: character.dexterity,
                name: "Stealth".to_string(),
                related_ability: Some("Dex".to_string()),
            },
            Skill {
                has_proficiency: false,
                ability_score: character.wisdom,
                name: "Survival".to_string(),
                related_ability: Some("Wis".to_string()),
            },
        ];

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
