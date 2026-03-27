use crate::{Entity, Player, db::Persistable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Campaign {
    id: i32,
    name: String,
    description: String,
    players: Vec<Player>,
    entities: Vec<Entity>,
}

impl Campaign {
    pub fn new(id: i32, name: String, description: String) -> Campaign {
        Campaign {
            id,
            name,
            description,
            players: vec![],
            entities: vec![],
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    pub fn player(&self, id: i32) -> Option<&Player> {
        self.players.iter().find(|p| p.id() == id)
    }

    pub fn players(&self) -> &[Player] {
        &self.players
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn entity(&self, id: i32) -> Option<&Entity> {
        self.entities.iter().find(|e| e.id() == id)
    }

    pub fn entities(&self) -> &[Entity] {
        &self.entities
    }
}

impl Persistable for Campaign {}
