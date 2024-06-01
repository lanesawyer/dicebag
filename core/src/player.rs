pub struct Player {
    id: i32,
    name: String,
    // Collection of stuff
}

impl Player {
    pub fn new(id: i32, name: String) -> Player {
        Player {
            id,
            name,
            // Collection of stuff
        }
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
