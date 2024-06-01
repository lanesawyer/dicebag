pub struct Campaign {
    id: i32,
    name: String,
    description: String,
}

impl Campaign {
    pub fn new(id: i32, name: String, description: String) -> Campaign {
        Campaign {
            id,
            name,
            description,
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
}
