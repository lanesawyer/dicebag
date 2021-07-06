use diesel::Queryable;
use juniper::{GraphQLObject, GraphQLInputObject };

#[derive(Clone, Default, GraphQLObject, Queryable)]
pub struct Character {
    pub id: i32,
    pub image: String,

    // Info
    pub name: String,
    pub class: String, // TODO: enum
    pub level: i32,
    pub background: String,
    pub race: String,      // TODO: enum?
    pub alignment: String, // TODO: enum
    pub experience_points: i32,

    // Stats
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub charisma: i32,

    // Other
    pub proficiency_bonus: i32,
    pub has_inspiration: bool,
    pub personality_traits: String,
    pub ideals: String,
    pub bonds: String,
    pub flaws: String,
    pub features_and_traits: String,
    pub other_proficiencies_and_languages: String,
    pub armor_class: i32,
    pub speed: i32,
    pub hit_points: i32,
    pub current_hit_points: i32,
    pub temporary_hit_points: i32,
    pub hit_dice: i32,
    pub used_hit_dice: i32,
    pub saves: i32,
    pub failures: i32,
    pub equipment: String,
    pub copper: i32,
    pub silver: i32,
    pub electrum: i32,
    pub platinum: i32,
    pub gold: i32,
}

impl Character {
    pub fn _new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl From<NewCharacter> for Character {
    fn from(new_character: NewCharacter) -> Self {
        Character { 
            id: new_character.id,
            name: new_character.name,
            ..Default::default()
        }
    }
}


#[derive(GraphQLInputObject)]
pub struct NewCharacter {
    id: String,
    image: Option<String>,
    name: String,
    class: String,
}
