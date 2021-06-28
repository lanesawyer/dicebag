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
                        id: "varis".to_string(),
                        name: "Varis".to_string(),
                        class: "Sorcerer".to_string(),
                        level: 4,
                        ..Default::default()
                    },
                ),
                (
                    2,
                    Character {
                        id: "campton".to_string(),
                        name: "Campton".to_string(),
                        class: "Fighter".to_string(),
                        level: 6,
                        ..Default::default()
                    },
                ),
            ]
            .iter()
            .cloned()
            .collect(),
        }
    }
}

// To make our context usable by Juniper, we have to implement a marker trait.
impl Context for Database {}
