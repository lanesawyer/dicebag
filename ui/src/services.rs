// use anyhow::Error;
use graphql_client::GraphQLQuery;
use serde::Deserialize;
// use serde_json::json;
// use yew::{
//     format::Json,
//     services::{
//         fetch::{FetchTask, Request, Response},
//         FetchService,
//     },
//     Component, ComponentLink,
// };

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries.graphql"
)]
pub struct CharactersQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.json",
    query_path = "src/graphql/queries.graphql"
)]
pub struct NewCharacterMutation;

// TODO: I should be able to use the auto-generated ones,
// but I'm running into deserialization issues with Yew's Fetch
#[derive(Debug, Deserialize)]
pub struct GraphQLResponse<T> {
    pub data: T,
}

// Trying to make a nice helper
// pub(crate) fn post<Comp, Resp, F, M,>(
//     query: graphql_client::QueryBody<characters_query::Variables>,
//     link: ComponentLink<Comp>,
//     receive_response: F,
// ) -> FetchTask
// where
//     Comp: Component,
//     Resp: From<Result<std::string::String, anyhow::Error>>,
//     F: Fn(anyhow::Result<Resp>) -> M + 'static,
//     M: Into<Comp::Message>,
// {
//     let request_json = &json!(query);

//     let request = Request::post("http://127.0.0.1:8000/graphql")
//         .header("Content-Type", "application/json")
//         .body(Json(request_json))
//         .expect("Could not build that request.");

//     let callback = link.callback(|response: Response<Json<anyhow::Result<Resp>>>| {
//         let Json(data) = response.into_body();
//         receive_response(data)
//     });

//     FetchService::fetch(request, callback).expect("failed to start request")
// }
