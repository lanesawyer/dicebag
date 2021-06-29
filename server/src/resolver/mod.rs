use crate::context::Database;
use crate::schema::character::Character;
use juniper::{graphql_object, FieldResult};

pub struct Query;

#[graphql_object(context = Database)]
impl Query {
    pub fn characters(context: &Database) -> FieldResult<Vec<Character>> {
        Ok(context.characters.values().cloned().collect::<Vec<_>>())
    }
}
