use crate::{Player, db::Persistable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Campaign {
    id: i32,
    name: String,
    description: String,
    players: Vec<Player>,
}

impl Campaign {
    pub fn new(id: i32, name: String, description: String) -> Campaign {
        Campaign {
            id,
            name,
            description,
            players: vec![],
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
}

impl Persistable for Campaign {}
