use graphql_client::{GraphQLQuery, Response};
use serde::Deserialize;
use serde_json::json;
use wasm_bindgen_futures::spawn_local;
use yew::{use_effect_with_deps, use_state};

use super::{build_request, GraphQLResponse};

#[derive(Clone)]
pub struct QueryResponse<T> {
    pub data: Option<T>,
    pub error: Option<String>,
}

// TODO: Use Q::ResponseData instead of another type
pub fn use_query<Q, Resp>(variables: Q::Variables) -> QueryResponse<Resp>
where
    Q: GraphQLQuery + 'static,
    Resp: for<'de> Deserialize<'de> + Clone + 'static,
{
    let state = use_state(|| QueryResponse {
        data: None,
        error: None,
    });

    let effect_state = state.clone();

    use_effect_with_deps(
        move |_| {
            spawn_local(async move {
                let request_body = Q::build_query(variables);
                let request_json = &json!(request_body);
                let request = build_request(request_json).await;
                match request {
                    Ok(response) => {
                        let json = response.json::<GraphQLResponse<Resp>>().await;
                        match json {
                            Ok(response) => effect_state.set(QueryResponse {
                                data: Some(response.data),
                                error: None,
                            }),
                            Err(error) => effect_state.set(QueryResponse {
                                data: None,
                                error: Some(error.to_string()),
                            }),
                        }
                    }
                    Err(error) => effect_state.set(QueryResponse {
                        data: None,
                        error: Some(error.to_string()),
                    }),
                }
            });

            || ()
        },
        (),
    );

    (*state).clone()
}

pub fn use_query_improved<Q>(variables: Q::Variables) -> QueryResponse<Q::ResponseData>
where
    Q: GraphQLQuery,
    Q::Variables: 'static,
    Q::ResponseData: Clone + 'static,
{
    let state = use_state(|| QueryResponse {
        data: None,
        error: None,
    });

    let effect_state = state.clone();

    use_effect_with_deps(
        move |_| {
            spawn_local(async move {
                let request_body = Q::build_query(variables);
                let request_json = &json!(request_body);
                let request = build_request(request_json).await;
                match request {
                    Ok(response) => {
                        let json = response.json::<Response<Q::ResponseData>>().await;
                        match json {
                            Ok(response) => effect_state.set(QueryResponse {
                                data: response.data,
                                error: None,
                            }),
                            Err(error) => effect_state.set(QueryResponse {
                                data: None,
                                error: Some(error.to_string()),
                            }),
                        }
                    }
                    Err(error) => effect_state.set(QueryResponse {
                        data: None,
                        error: Some(error.to_string()),
                    }),
                }
            });

            || ()
        },
        (),
    );

    (*state).clone()
}
