use crate::schema::character::Character;
use juniper::Context;
use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct Database {
    // TODO: Use a real database pool here later
    pub characters: HashMap<i32, Character>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            characters: [
                (
                    1,
                    Character {
                        id: "guid".to_string(),
                        name: "Varis".to_string(),
                        ..Default::default()
                    },
                ),
                (2, Character::new()),
            ]
            .iter()
            .cloned()
            .collect(),
        }
    }
}

// To make our context usable by Juniper, we have to implement a marker trait.
impl Context for Database {}
