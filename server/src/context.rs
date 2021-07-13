use diesel_migrations::embed_migrations;
use juniper::Context;
use rocket::{Build, Rocket};
use rocket_sync_db_pools::database;

#[database("postgres")]
pub struct Database(diesel::PgConnection);

impl Context for Database {}

pub async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    embed_migrations!();

    let conn = Database::get_one(&rocket)
        .await
        .expect("Couldn't connect to the database.");

    conn.run(|c| embedded_migrations::run(c))
        .await
        .expect("Failed to run diesel migrations");

    rocket
}
