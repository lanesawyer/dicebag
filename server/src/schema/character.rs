use juniper::GraphQLObject;

#[derive(Clone, Default, GraphQLObject)]
pub struct Character {
    pub id: String,

    // Info
    pub name: String,
    pub class: String, // TODO: enum
    pub level: i32,
    pub background: String,
    pub race: String,      // TODO: enum?
    pub alignment: String, // TODO: enum
    pub experience_points: i32,

    // Stats
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub charisma: i32,

    // Other
    pub proficiency_bonus: i32,
    pub has_inspiration: bool,
    pub personality_traits: String,
    pub ideals: String,
    pub bonds: String,
    pub flaws: String,
    pub features_and_traits: String,
    pub other_proficiencies_and_languages: String,
    pub armor_class: i32,
    pub speed: i32,
    pub hit_points: i32,
    pub current_hit_points: i32,
    pub temporary_hit_points: i32,
    pub hit_dice: i32,
    pub used_hit_dice: i32,
    pub saves: i32,
    pub failures: i32,
    pub equipment: String,
    pub copper: i32,
    pub silver: i32,
    pub electrum: i32,
    pub platinum: i32,
    pub gold: i32,
}

impl Character {
    pub fn _new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
