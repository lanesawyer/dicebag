use serde::Deserialize;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use yew::{use_effect_with_deps, use_state};

use super::{build_request, GraphQLResponse};

#[derive(Clone)]
pub struct QueryResponse<T> {
    pub data: Option<T>,
    pub error: Option<String>,
}

// TODO: Make even more generic so we can pass in the variables and the query we want to use instead of the serialized JSON
pub fn use_query<T>(request_json: &Value) -> QueryResponse<T>
where
    T: for<'de> Deserialize<'de> + Clone + 'static,
{
    let state = use_state(|| QueryResponse {
        data: None,
        error: None,
    });

    let effect_state = state.clone();

    let request_json = request_json.clone();
    use_effect_with_deps(
        move |_| {
            spawn_local(async move {
                let request = build_request(&request_json).await;
                match request {
                    Ok(response) => {
                        let json = response.json::<GraphQLResponse<T>>().await;
                        match json {
                            Ok(responser) => effect_state.set(QueryResponse {
                                data: Some(responser.data),
                                error: None,
                            }),
                            Err(error) => effect_state.set(QueryResponse {
                                data: None,
                                error: Some(error.to_string()),
                            }),
                        }
                    },
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
