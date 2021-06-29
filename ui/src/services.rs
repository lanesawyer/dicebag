use graphql_client::GraphQLQuery;
use serde::Deserialize;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries.graphql"
)]
pub struct CharactersQuery;

// TODO: I should be able to use the auto-generated ones,
// but I'm running into deserialization issues with Yew's Fetch
#[derive(Debug, Deserialize)]
pub struct GraphQLResponse<T> {
    pub data: T,
}
