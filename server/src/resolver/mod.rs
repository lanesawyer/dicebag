extern crate diesel;

use crate::{context::Database, schema::character::Character};
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
