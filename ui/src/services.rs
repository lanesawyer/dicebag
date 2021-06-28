use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(schema_path = "src/schema.json", query_path = "src/queries.graphql")]
pub struct CharacterQuery;
