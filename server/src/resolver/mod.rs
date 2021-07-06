extern crate diesel;

use crate::{context::Database, schema::character::Character};
use crate::schema::character::{Character, NewCharacter};
use juniper::{graphql_object, FieldResult};

use self::diesel::prelude::*;

pub struct Query;

#[graphql_object(context = Database)]
impl Query {
    pub async fn characters(context: &Database) -> FieldResult<Vec<Character>> {
        use crate::schema::db::characters::dsl::*;
        let results = context
            .run(|c| {
                characters
                    .load::<Character>(c)
                    .expect("Error loading characters")
            })
            .await;

        Ok(results)
    }
}

pub struct Mutation;

#[graphql_object(context = Database)]
impl Mutation {
    pub fn add_character(context: &mut Database, _new_character: NewCharacter) -> FieldResult<&Character> {
        // context.characters.insert(2, new_character.into());

        let character = context.characters.get(&2).unwrap();
        Ok(character)
    }
}
