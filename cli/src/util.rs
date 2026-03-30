use std::path::PathBuf;

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

pub fn campaign_file(dir: &PathBuf) -> PathBuf {
    dir.join("campaign.ron")
}

pub fn players_file(dir: &PathBuf) -> PathBuf {
    dir.join("players.ron")
}

pub fn entities_file(dir: &PathBuf) -> PathBuf {
    dir.join("entities.ron")
}

pub fn encounter_file(dir: &PathBuf, encounter_id: i32) -> PathBuf {
    dir.join(format!("encounter-{}.ron", encounter_id))
}

pub fn audio_catalog_file(dir: &PathBuf) -> PathBuf {
    dir.join("audio-catalog.ron")
}

pub fn audio_file(dir: &PathBuf, recording_id: i32) -> PathBuf {
    dir.join(format!("audio-{}.webm", recording_id))
}
