#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use juniper::{EmptyMutation, EmptySubscription, IntrospectionFormat, RootNode};
use rocket::{response::content, Build, Rocket, State};

use crate::context::Database;
use crate::resolver::Query;

mod context;
mod resolver;
mod schema;

type Schema = RootNode<'static, Query, EmptyMutation<Database>, EmptySubscription<Database>>;

#[rocket::get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket_async::graphiql_source("/graphql", None)
}

#[rocket::get("/graphql?<request>")]
fn get_graphql_handler(
    context: &State<Database>,
    request: juniper_rocket_async::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket_async::GraphQLResponse {
    request.execute_sync(&*schema, &*context)
}

#[rocket::post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: &State<Database>,
    request: juniper_rocket_async::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket_async::GraphQLResponse {
    request.execute_sync(&*schema, &*context)
}

#[rocket::post("/")]
fn introspection_handler(context: &State<Database>) -> content::Json<String> {
    let (res, _errors) = juniper::introspect(
        &Schema::new(Query, EmptyMutation::new(), EmptySubscription::new()),
        context,
        IntrospectionFormat::default(),
    )
    .unwrap();

    content::Json(serde_json::to_string_pretty(&res).unwrap())
}

#[launch]
async fn rocket() -> Rocket<Build> {
    dotenv().ok();

    rocket::build()
        .manage(Database::new())
        .manage(Schema::new(
            Query,
            EmptyMutation::<Database>::new(),
            EmptySubscription::<Database>::new(),
        ))
        .mount(
            "/",
            rocket::routes![
                graphiql,
                get_graphql_handler,
                post_graphql_handler,
                introspection_handler
            ],
        )
}
