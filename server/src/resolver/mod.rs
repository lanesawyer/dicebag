extern crate diesel;

use crate::schema::campaign::{Campaign, NewCampaign};
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

    pub async fn campaigns(context: &Database) -> FieldResult<Vec<Campaign>> {
        use crate::schema::db::campaigns::dsl::*;
        let results = context
            .run(|c| {
                campaigns
                    .load::<Campaign>(c)
                    .expect("Error loading campaigns")
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
                "Unable to delete character",
                graphql_value!({ "internal_error": "Database delete failed" }),
            )),
        }
    }

    pub async fn new_campaign(context: &Database, new_campaign: NewCampaign) -> FieldResult<bool> {
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

    pub async fn delete_campaign(context: &Database, delete_id: i32) -> FieldResult<bool> {
        use crate::schema::db::campaigns::dsl::*;
        use crate::schema::db::characters::dsl::{campaign_id, characters};

        // TODO: Clean up
        match context
            .run(move |c| {
                let characters_target = characters.filter(campaign_id.eq(delete_id));
                diesel::update(characters_target)
                    .set(campaign_id.eq(None::<i32>))
                    .execute(c)?;
                diesel::delete(campaigns.filter(id.eq(delete_id))).execute(c)
            })
            .await
        {
            Ok(_) => Ok(true),
            Err(_) => Err(FieldError::new(
                "Unable to delete campaign",
                graphql_value!({ "internal_error": "Database delete failed" }),
            )),
        }
    }
}
