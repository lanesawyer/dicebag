// use graphql_client::GraphQLQuery;
// use serde_json::json;
use yew::{
    // format::Json,
    html,
    // services::{
    //     fetch::{Request, Response},
    //     ConsoleService, FetchService,
    // },
    Component,
    ComponentLink,
    Html,
    ShouldRender,
};

// #[derive(GraphQLQuery)]
// #[graphql(schema_path = "src/schema.json", query_path = "src/queries.graphql")]
// struct CharactersQuery;

pub struct CharactersPage;

impl Component for CharactersPage {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        // let variables = characters_query::Variables {};
        // let request_body = CharactersQuery::build_query(variables);
        // let request_json = &json!(request_body);

        // ConsoleService::log(&format!("{:?}", &request_json));

        // // TODO: Pull URL from .env
        // let request = Request::post("http://127.0.0.1:8000/graphql")
        //     .header("Content-Type", "application/json")
        //     .body(Json(request_json))
        //     .expect("Could not build that request.");

        // let callback = link.callback(
        //     |response: Response<Json<Result<GraphQLResponse<CharacterList>, anyhow::Error>>>| {
        //         let Json(data) = response.into_body();
        //         Msg::ReceiveResponse(data)
        //     },
        // );

        // let task = FetchService::fetch(request, callback).expect("failed to start request");

        // Self {
        //     props,
        //     character: Some(build_bob()),
        //     link,
        //     fetch_task: Some(task),
        //     error: None,
        // }
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                { "Characters here" }
            </>
        }
    }
}
