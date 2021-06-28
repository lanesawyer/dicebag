use graphql_client::GraphQLQuery;
use serde::Deserialize;

#[derive(GraphQLQuery)]
#[graphql(schema_path = "src/schema.json", query_path = "src/queries.graphql")]
pub struct CharacterQuery;

// TODO: I should be able to use the auto-generated ones,
// but I'm running into deserialization issues with Yew's Fetch
#[derive(Debug, Deserialize)]
pub struct GraphQLResponse<T> {
    pub data: T,
}
