use core::{CombatantRef, EntityRoster, PlayerRoster};

pub fn resolve_name<'a>(
    source: &CombatantRef,
    players: &'a PlayerRoster,
    entities: &'a EntityRoster,
) -> &'a str {
    match source {
        CombatantRef::Player(id) => players
            .get(*id)
            .map(|p| p.name())
            .unwrap_or("Unknown Player"),
        CombatantRef::Entity(id) => entities
            .get(*id)
            .map(|e| e.name())
            .unwrap_or("Unknown Entity"),
    }
}

pub fn encounter_filename(name: &str) -> String {
    format!("{}.ron", name.to_lowercase().replace(' ', "-"))
}

pub fn campaign_filename(name: &str) -> String {
    format!("{}.ron", name.to_lowercase().replace(' ', "-"))
}

pub fn players_filename(campaign_name: &str) -> String {
    format!(
        "{}-players.ron",
        campaign_name.to_lowercase().replace(' ', "-")
    )
}

pub fn entities_filename(campaign_name: &str) -> String {
    format!(
        "{}-entities.ron",
        campaign_name.to_lowercase().replace(' ', "-")
    )
}
