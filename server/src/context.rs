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

// #[derive(Default, Clone)]
// pub struct Database {
//     // TODO: Use a real database pool here later
//     pub characters: HashMap<i32, Character>,
// }

// impl Database {
//     pub fn new() -> Self {
//         Self {
//             characters: [
//                 (
//                     1,
//                     Character {
//                         id: 1,
//                         image: "https://i.pinimg.com/originals/cf/92/f1/cf92f180a34a7042bdecf7b223a02f1e.png".to_string(),
//                         name: "Varis".to_string(),
//                         class: "Sorcerer".to_string(),
//                         level: 4,
//                         ..Default::default()
//                     },
//                 ),
//                 (
//                     2,
//                     Character {
//                         id: 2,
//                         image: "https://i.pinimg.com/originals/5f/0d/cb/5f0dcb7e87d712f8adb270b9cf4b264c.jpg".to_string(),
//                         name: "Campton".to_string(),
//                         class: "Fighter".to_string(),
//                         level: 6,
//                         ..Default::default()
//                     },
//                 ),
//             ]
//             .iter()
//             .cloned()
//             .collect(),
//         }
//     }
// }

// // To make our context usable by Juniper, we have to implement a marker trait.
// impl Context for Database {}
