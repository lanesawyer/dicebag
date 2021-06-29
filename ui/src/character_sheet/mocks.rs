use crate::character_sheet::sheet::Character;
use crate::components::Skill;

pub fn build_bob() -> Character {
    Character {
        id: "bob".to_string(),
        image: "https://wallpapercave.com/wp/wp2255801.jpg".to_string(),
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
        features_and_traits: "Organizer: Goes into battle with an extra action to plan".to_string(),
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
    }
}

pub fn build_skills(character: &Character) -> Vec<Skill> {
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
            ability_score: character.charisma,
            name: "Charisma".to_string(),
            related_ability: None,
        },
    ]
}

pub fn build_saving_throws(character: &Character) -> Vec<Skill> {
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
    ]
}
