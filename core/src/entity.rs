use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum EntityKind {
    Enemy,
    AllyNpc,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Entity {
    id: i32,
    name: String,
    kind: EntityKind,
    max_hp: i32,
    notes: String,
}

impl Entity {
    pub fn new(id: i32, name: String, kind: EntityKind, max_hp: i32) -> Self {
        Entity {
            id,
            name,
            kind,
            max_hp,
            notes: String::new(),
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn kind(&self) -> &EntityKind {
        &self.kind
    }

    pub fn max_hp(&self) -> i32 {
        self.max_hp
    }

    pub fn notes(&self) -> &str {
        &self.notes
    }

    pub fn set_notes(&mut self, notes: String) {
        self.notes = notes;
    }
}
