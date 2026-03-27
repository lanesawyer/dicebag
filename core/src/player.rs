use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    id: i32,
    name: String,
    // Collection of stuff
}

impl Player {
    pub fn new(id: i32, name: String) -> Player {
        Player { id, name }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct PlayerRoster {
    players: Vec<Player>,
}

impl PlayerRoster {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn players(&self) -> &[Player] {
        &self.players
    }

    pub fn get(&self, id: i32) -> Option<&Player> {
        self.players.iter().find(|p| p.id == id)
    }

    pub fn add(&mut self, player: Player) {
        self.players.push(player);
    }

    pub fn update_name(&mut self, id: i32, name: String) -> bool {
        match self.players.iter_mut().find(|p| p.id == id) {
            Some(p) => {
                p.set_name(name);
                true
            }
            None => false,
        }
    }

    pub fn remove(&mut self, id: i32) -> bool {
        let before = self.players.len();
        self.players.retain(|p| p.id != id);
        self.players.len() < before
    }

    pub fn next_id(&self) -> i32 {
        self.players.iter().map(|p| p.id).max().unwrap_or(0) + 1
    }
}
