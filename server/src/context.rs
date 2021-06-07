use crate::schema::character::Character;
use std::collections::HashMap;

pub struct Context {
    // TODO: Use a real database pool here later
    pub characters: HashMap<i32, Character>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            characters: [(
                1,
                Character {
                    id: "guid".to_string(),
                    name: "Varis".to_string(),
                    ..Default::default()
                },
            )]
            .iter()
            .cloned()
            .collect(),
        }
    }
}

// To make our context usable by Juniper, we have to implement a marker trait.
impl juniper::Context for Context {}
