use crate::diesel::prelude::*;
use diesel::{result::Error, PgConnection, Queryable};
use juniper::{GraphQLInputObject, GraphQLObject};

use crate::schema::db::campaigns;

#[derive(Clone, Default, GraphQLObject,  Queryable, Insertable)]
pub struct Campaign {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

impl Campaign {
    // TODO: Figure out how to do this with db pools
    pub async fn get_all(conn: &PgConnection) -> Result<Vec<Self>, Error> {
        use crate::schema::db::campaigns::dsl::*;
        campaigns.load::<Campaign>(conn)
    }

    // TODO: Figure out how to do this with db pools
    pub async fn create(conn: &PgConnection, new_campaign: NewCampaign) -> Result<Self, Error> {
        use crate::schema::db::campaigns::dsl::*;

        let campaign: Campaign = new_campaign.into();
        diesel::insert_into(campaigns)
            .values(campaign)
            .get_result(conn)
    }
}

impl From<NewCampaign> for Campaign {
    fn from(new_campaign: NewCampaign) -> Self {
        Campaign {
            name: new_campaign.name,
            ..Default::default()
        }
    }
}

#[derive(GraphQLInputObject, Insertable)]
#[table_name = "campaigns"]
pub struct NewCampaign {
    name: String,
    description: Option<String>,
}
