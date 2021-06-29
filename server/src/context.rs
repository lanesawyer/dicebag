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
                        image: "https://i.pinimg.com/originals/cf/92/f1/cf92f180a34a7042bdecf7b223a02f1e.png".to_string(),
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
                        image: "https://i.pinimg.com/originals/5f/0d/cb/5f0dcb7e87d712f8adb270b9cf4b264c.jpg".to_string(),
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
