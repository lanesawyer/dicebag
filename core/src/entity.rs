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

#[derive(Serialize, Deserialize, Default)]
pub struct EntityRoster {
    entities: Vec<Entity>,
}

impl EntityRoster {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn entities(&self) -> &[Entity] {
        &self.entities
    }

    pub fn get(&self, id: i32) -> Option<&Entity> {
        self.entities.iter().find(|e| e.id == id)
    }

    pub fn add(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn remove(&mut self, id: i32) -> bool {
        let before = self.entities.len();
        self.entities.retain(|e| e.id != id);
        self.entities.len() < before
    }

    pub fn next_id(&self) -> i32 {
        self.entities.iter().map(|e| e.id).max().unwrap_or(0) + 1
    }
}
