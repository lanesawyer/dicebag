use core::{Campaign, CombatantRef};

pub fn resolve_name<'a>(source: &CombatantRef, campaign: &'a Campaign) -> &'a str {
    match source {
        CombatantRef::Player(id) => campaign
            .player(*id)
            .map(|p| p.name())
            .unwrap_or("Unknown Player"),
        CombatantRef::Entity(id) => campaign
            .entity(*id)
            .map(|e| e.name())
            .unwrap_or("Unknown Entity"),
    }
}

pub fn encounter_filename(name: &str) -> String {
    format!("{}.ron", name.to_lowercase().replace(' ', "-"))
}
