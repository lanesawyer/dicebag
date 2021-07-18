use anyhow::Error;
use dotenv_codegen::dotenv;
// use anyhow::Error;
use graphql_client::GraphQLQuery;
use serde::Deserialize;
use serde_json::{Value, json};
use yew::{Callback, Component, ComponentLink, format::Json, services::{
        fetch::{FetchTask, Request, Response},
        FetchService,
    }};

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

pub fn build_request(request_json: &Value) -> Request<Json<&Value>> {
    let api_url = dotenv!("API_URL");
    Request::post(api_url)
        .header("Content-Type", "application/json")
        .body(Json(request_json))
        .expect("Could not build the request.")
}

// Trying to make a nice helper
// pub(crate) fn post<'a, Variables, T>(
//     query: graphql_client::QueryBody<Variables>,
//     callback: Callback<Response<Json<Result<GraphQLResponse<T>, Error>>>>,
// ) -> FetchTask
//     where Variables: serde::Serialize,
//     T: 'static + serde::Deserialize<'static>,
// {
//     let request_json = &json!(query);

//     let request = Request::post("http://127.0.0.1:8000/graphql")
//         .header("Content-Type", "application/json")
//         .body(Json(request_json))
//         .expect("Could not build that request.");

//     FetchService::fetch(request, callback).expect("failed to start request")
// }
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
