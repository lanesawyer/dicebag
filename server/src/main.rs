#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

use dotenv::dotenv;
use juniper::{EmptyMutation, EmptySubscription, IntrospectionFormat, RootNode};
use rocket::{response::content, Build, Rocket, State};

use context::Database;
use resolver::Query;

mod context;
mod resolver;
mod schema;

type Schema = RootNode<'static, Query, EmptyMutation<Database>, EmptySubscription<Database>>;

#[rocket::get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql", None)
}

#[rocket::get("/graphql?<request>")]
async fn get_graphql_handler(
    context: Database,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&*schema, &context).await
}

#[rocket::post("/graphql", data = "<request>")]
async fn post_graphql_handler(
    context: Database,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&*schema, &context).await
}

#[rocket::post("/")]
fn introspection_handler(context: Database) -> content::Json<String> {
    let (res, _errors) = juniper::introspect(
        &Schema::new(Query, EmptyMutation::new(), EmptySubscription::new()),
        &context,
        IntrospectionFormat::default(),
    )
    .unwrap();

    content::Json(serde_json::to_string_pretty(&res).unwrap())
}

#[launch]
async fn rocket() -> Rocket<Build> {
    dotenv().ok();

    rocket::build()
        .attach(Database::fairing())
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
