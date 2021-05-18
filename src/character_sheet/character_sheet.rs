use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

use crate::components::Skill;

use super::{
    armor_class::ArmorClass,
    attacks::Attacks,
    character_info::CharacterInfo,
    death_saving_throws::DeathSavingThrows,
    hit_dice::HitDice,
    hit_points::HitPoints,
    initiative::Initiative,
    inspiration::Inspiration,
    money::Money,
    passive_perception::PassivePerception,
    proficiency_bonus::ProficiencyBonus,
    saving_throws::SavingThrows,
    skills::Skills,
    speed::Speed,
    stat_block::StatBlock,
    text_block::TextBlock,
};

#[derive(Properties, Clone)]
pub struct CharacterSheet {
    pub character: Character,
}

#[derive(Clone)]
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
    type Message = ();
    type Properties = ();
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            character: Character {
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
            },
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let saving_throws = vec![
            Skill {
                has_proficiency: false,
                ability_score: self.character.strength,
                name: "Strength".to_string(),
                related_ability: None,
            },
            Skill {
                has_proficiency: false,
                ability_score: self.character.dexterity,
                name: "Dexterity".to_string(),
                related_ability: None,
            },
            Skill {
                has_proficiency: true,
                ability_score: self.character.constitution,
                name: "Constitution".to_string(),
                related_ability: None,
            },
            Skill {
                has_proficiency: false,
                ability_score: self.character.intelligence,
                name: "Intelligence".to_string(),
                related_ability: None,
            },
            Skill {
                has_proficiency: false,
                ability_score: self.character.charisma,
                name: "Charisma".to_string(),
                related_ability: None,
            },
        ];

        let skills = vec![
            Skill {
                has_proficiency: false,
                ability_score: self.character.dexterity,
                name: "Acrobatics".to_string(),
                related_ability: Some("Dex".to_string()),
            },
            Skill {
                has_proficiency: false,
                ability_score: self.character.wisdom,
                name: "Animal Handling".to_string(),
                related_ability: Some("Wis".to_string()),
            },
            Skill {
                has_proficiency: false,
                ability_score: self.character.intelligence,
                name: "Arcana".to_string(),
                related_ability: Some("Int".to_string()),
            },
            Skill {
                has_proficiency: false,
                ability_score: self.character.strength,
                name: "Athletics".to_string(),
                related_ability: Some("Str".to_string()),
            },
            Skill {
                has_proficiency: false,
                ability_score: self.character.charisma,
                name: "Deception".to_string(),
                related_ability: Some("Cha".to_string()),
            },
            Skill {
                has_proficiency: false,
                ability_score: self.character.intelligence,
                name: "History".to_string(),
                related_ability: Some("Int".to_string()),
            },
            Skill {
                has_proficiency: false,
                ability_score: self.character.wisdom,
                name: "Insight".to_string(),
                related_ability: Some("Wis".to_string()),
            },
            Skill {
                has_proficiency: false,
                ability_score: self.character.charisma,
                name: "Intimidation".to_string(),
                related_ability: Some("Char".to_string()),
            },
            Skill {
                has_proficiency: false,
                ability_score: self.character.intelligence,
                name: "Investigation".to_string(),
                related_ability: Some("Int".to_string()),
            },
            Skill {
                has_proficiency: false,
                ability_score: self.character.wisdom,
                name: "Medicine".to_string(),
                related_ability: Some("Wis".to_string()),
            },
            Skill {
                has_proficiency: false,
                ability_score: self.character.intelligence,
                name: "Nature".to_string(),
                related_ability: Some("Int".to_string()),
            },
            Skill {
                has_proficiency: false,
                ability_score: self.character.wisdom,
                name: "Perception".to_string(),
                related_ability: Some("Wis".to_string()),
            },
            Skill {
                has_proficiency: false,
                ability_score: self.character.charisma,
                name: "Performance".to_string(),
                related_ability: Some("Cha".to_string()),
            },
            Skill {
                has_proficiency: false,
                ability_score: self.character.charisma,
                name: "Persuasion".to_string(),
                related_ability: Some("Cha".to_string()),
            },
            Skill {
                has_proficiency: false,
                ability_score: self.character.intelligence,
                name: "Religion".to_string(),
                related_ability: Some("Int".to_string()),
            },
            Skill {
                has_proficiency: false,
                ability_score: self.character.dexterity,
                name: "Sleight of Hand".to_string(),
                related_ability: Some("Dex".to_string()),
            },
            Skill {
                has_proficiency: false,
                ability_score: self.character.dexterity,
                name: "Stealth".to_string(),
                related_ability: Some("Dex".to_string()),
            },
            Skill {
                has_proficiency: false,
                ability_score: self.character.wisdom,
                name: "Survival".to_string(),
                related_ability: Some("Wis".to_string()),
            },
        ];

        html! {
            <>
                <CharacterInfo
                    name={&self.character.name}
                    class={&self.character.class}
                    level={&self.character.level}
                    background={&self.character.background}
                    race={&self.character.race}
                    alignment={&self.character.alignment}
                    experience_points={self.character.experience_points}
                />
                <section id="stat-block" class="stats">
                    <StatBlock name="Strength" value={self.character.strength} />
                    <StatBlock name="Dexterity" value={self.character.dexterity} />
                    <StatBlock name="Constitution" value={self.character.constitution} />
                    <StatBlock name="Intelligence" value={self.character.intelligence} />
                    <StatBlock name="Wisdom" value={self.character.wisdom} />
                    <StatBlock name="Charisma" value={self.character.charisma} />
                </section>
                <Inspiration value=self.character.has_inspiration />
                <ProficiencyBonus value=self.character.proficiency_bonus />
                <SavingThrows items=saving_throws />
                <Skills items=skills />
                <PassivePerception value=1 />
                <section id="other-proficiencies-and-languages">
                    <TextBlock name="Other Proficiencies & Languages" value=&self.character.other_proficiencies_and_languages />
                </section>
                <ArmorClass value=self.character.armor_class />
                <Initiative value=self.character.dexterity />
                <Speed value=self.character.speed />
                <HitPoints
                    maximum=self.character.hit_points
                    current=self.character.current_hit_points
                    temporary=self.character.temporary_hit_points
                />
                <HitDice total=self.character.hit_dice used=self.character.used_hit_dice />
                <DeathSavingThrows saves=self.character.saves failures=self.character.failures />
                <Attacks attacks=Vec::new() />
                <Money
                    copper=self.character.copper
                    silver=self.character.silver
                    electrum=self.character.electrum
                    gold=self.character.gold
                    platinum=self.character.platinum
                />
                <section id="equipment">
                    <TextBlock name="Equipment" value=&self.character.equipment />
                </section>
                <section id="character-details">
                    <TextBlock name="Personality Traits" value=&self.character.personality_traits />
                    <TextBlock name="Ideals" value=&self.character.ideals />
                    <TextBlock name="Bonds" value=&self.character.bonds />
                    <TextBlock name="Flaws" value=&self.character.flaws />
                </section>
                <section id="features-traits">
                    <TextBlock name="Features & Traits" value=&self.character.features_and_traits />
                </section>

            </>
        }
    }
}
