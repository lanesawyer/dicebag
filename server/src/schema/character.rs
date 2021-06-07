use juniper::{GraphQLObject};

#[derive(Clone, GraphQLObject)]
pub struct Character {
    pub id: i32, // this field doesn't exist on current schema
    pub name: String,
}
