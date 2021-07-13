use crate::diesel::prelude::*;
use diesel::{result::Error, PgConnection, Queryable};
use juniper::{GraphQLInputObject, GraphQLObject};

use crate::schema::db::characters;

#[derive(Clone, Default, GraphQLObject, Queryable, Insertable)]
pub struct Character {
    pub id: i32,
    pub image: Option<String>,

    // Info
    pub name: String,
    pub class: String, // TODO: enum
    pub level: i32,
    pub background: Option<String>,
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

    pub personality_traits: Option<String>,
    pub ideals: Option<String>,
    pub bonds: Option<String>,
    pub flaws: Option<String>,
    pub features_and_traits: Option<String>,
    pub other_proficiencies_and_languages: Option<String>,

    pub armor_class: i32,
    pub speed: i32,
    pub hit_points: i32,
    pub current_hit_points: i32,
    pub temporary_hit_points: i32,
    pub hit_dice: i32,
    pub used_hit_dice: i32,
    pub saves: i32,
    pub failures: i32,
    pub equipment: Option<String>,
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

    // TODO: Figure out how to do this with db pools
    pub async fn get_all(conn: &PgConnection) -> Result<Vec<Self>, Error> {
        use crate::schema::db::characters::dsl::*;
        characters.load::<Character>(conn)
    }

    // TODO: Figure out how to do this with db pools
    pub async fn create(conn: &PgConnection, new_character: NewCharacter) -> Result<Self, Error> {
        use crate::schema::db::characters::dsl::*;

        let character: Character = new_character.into();
        diesel::insert_into(characters)
            .values(character)
            .get_result(conn)
    }
}

impl From<NewCharacter> for Character {
    fn from(new_character: NewCharacter) -> Self {
        Character {
            id: new_character.id,
            image: new_character.image,
            name: new_character.name,
            class: new_character.class,
            race: new_character.race,
            alignment: new_character.alignment,
            strength: new_character.strength,
            dexterity: new_character.dexterity,
            constitution: new_character.constitution,
            intelligence: new_character.intelligence,
            wisdom: new_character.wisdom,
            charisma: new_character.charisma,
            ..Default::default()
        }
    }
}

#[derive(GraphQLInputObject)]
pub struct NewCharacter {
    id: i32,
    image: Option<String>,
    name: String,
    class: String,
    race: String,
    alignment: String,
    strength: i32,
    dexterity: i32,
    constitution: i32,
    intelligence: i32,
    wisdom: i32,
    charisma: i32,
}
