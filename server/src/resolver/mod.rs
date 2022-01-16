extern crate diesel;

use crate::schema::campaign::NewCampaign;
use crate::schema::character::NewCharacter;
use crate::{context::Database, schema::character::Character};
use juniper::{graphql_object, graphql_value, FieldError, FieldResult};

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
    pub async fn new_character(
        context: &Database,
        new_character: NewCharacter,
    ) -> FieldResult<bool> {
        use crate::schema::db::characters::dsl::*;

        // TODO: Clean up
        match context
            .run(|c| {
                diesel::insert_into(characters)
                    .values(new_character)
                    .execute(c)
            })
            .await
        {
            Ok(_) => Ok(true),
            Err(_) => Err(FieldError::new(
                "Unable to create character",
                graphql_value!({ "internal_error": "Database insert failed" }),
            )),
        }
    }

    pub async fn delete_character(context: &Database, delete_id: i32) -> FieldResult<bool> {
        use crate::schema::db::characters::dsl::*;

        // TODO: Clean up
        match context
            .run(move |c| diesel::delete(characters.filter(id.eq(delete_id))).execute(c))
            .await
        {
            Ok(_) => Ok(true),
            Err(_) => Err(FieldError::new(
                "Unable to create character",
                graphql_value!({ "internal_error": "Database delete failed" }),
            )),
        }
    }

    pub async fn new_campaign(
        context: &Database,
        new_campaign: NewCampaign,
    ) -> FieldResult<bool> {
        use crate::schema::db::campaigns::dsl::*;

        // TODO: Clean up
        match context
            .run(|c| {
                diesel::insert_into(campaigns)
                    .values(new_campaign)
                    .execute(c)
            })
            .await
        {
            Ok(_) => Ok(true),
            Err(_) => Err(FieldError::new(
                "Unable to create campaign",
                graphql_value!({ "internal_error": "Database insert failed" }),
            )),
        }
    }
}
