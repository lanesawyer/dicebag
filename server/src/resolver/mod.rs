use crate::context::Context;
use crate::schema::character::Character;
use juniper::{graphql_object, FieldResult};

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    pub fn character(
        context: &Context,
    ) -> FieldResult<Vec<Character>> {
        Ok(context.characters.values().cloned().collect::<Vec<_>>())
    }
}
