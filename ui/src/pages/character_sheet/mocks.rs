use crate::components::Skill;
use crate::services::characters_query::CharactersQueryCharacters;

pub fn _build_bob() -> CharactersQueryCharacters {
    CharactersQueryCharacters {
        id: 1,
        image: Some("https://wallpapercave.com/wp/wp2255801.jpg".to_string()),
        name: "Bob the Builder".to_string(),
        class: "Artificer".to_string(),
        level: 3,
        background: Some("Construction worker".to_string()),
        race: "Human".to_string(),
        alignment: Some("Lawful Good".to_string()),
        experience_points: 1800,
        strength: 12,
        dexterity: 8,
        constitution: 14,
        intelligence: 10,
        wisdom: 12,
        charisma: 8,
        proficiency_bonus: 1,
        has_inspiration: false,
        personality_traits: Some("Happy go lucky".to_string()),
        ideals: Some("A clean site is a safe site".to_string()),
        bonds: Some("I will always build a solid foundation".to_string()),
        flaws: Some("Talks to machines".to_string()),
        features_and_traits: Some(
            "Organizer: Goes into battle with an extra action to plan".to_string(),
        ),
        other_proficiencies_and_languages: Some("English and Spanish".to_string()),
        armor_class: 13,
        hit_points: 20,
        current_hit_points: 18,
        temporary_hit_points: 0,
        hit_dice: 3,
        used_hit_dice: 0,
        saves: 2,
        failures: 1,
        speed: 30,
        equipment: Some("Hammer".to_string()),
        copper: 10,
        silver: 5,
        electrum: 1,
        gold: 3,
        platinum: 0,
    }
}

// TODO: Get from server
pub fn build_saving_throws(character: &CharactersQueryCharacters) -> Vec<Skill> {
    vec![
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
            ability_score: character.wisdom,
            name: "Wisdom".to_string(),
            related_ability: None,
        },
        Skill {
            has_proficiency: false,
            ability_score: character.charisma,
            name: "Charisma".to_string(),
            related_ability: None,
        },
    ]
}

// TODO: Get from server
pub fn build_skills(character: &CharactersQueryCharacters) -> Vec<Skill> {
    vec![
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
            related_ability: Some("Cha".to_string()),
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
    ]
}
