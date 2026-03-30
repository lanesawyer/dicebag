use clap::Subcommand;
use core::{Campaign, Encounter, EntityRoster, Participant, PlayerRoster};

use crate::persistence::{find_campaign_dir, load, save};
use crate::util::{campaign_file, encounter_file, entities_file, players_file, resolve_name};

#[derive(Subcommand)]
pub enum EncounterCommands {
    /// Create a new encounter for a campaign
    New {
        #[arg(short, long)]
        name: String,
        /// Campaign name (e.g. "My Campaign")
        #[arg(short, long)]
        campaign: String,
    },
    /// Add a participant to an encounter (player or entity by id)
    AddParticipant {
        /// Campaign name (e.g. "My Campaign")
        #[arg(short, long)]
        campaign: String,
        /// Encounter id
        #[arg(short, long)]
        encounter: i32,
        /// ID of the player or entity to add
        #[arg(long)]
        id: i32,
        /// "player" or "entity"
        #[arg(long)]
        kind: String,
        /// Override max HP for this participant (optional)
        #[arg(long)]
        max_hp: Option<i32>,
    },
    /// Set initiative for a participant in an encounter
    SetInitiative {
        /// Campaign name (e.g. "My Campaign")
        #[arg(short, long)]
        campaign: String,
        /// Encounter id
        #[arg(short, long)]
        encounter: i32,
        /// Participant id
        #[arg(long)]
        id: i32,
        #[arg(long)]
        initiative: i32,
    },
    /// Apply damage or healing to a participant (positive = damage, negative = healing)
    HpChange {
        /// Campaign name (e.g. "My Campaign")
        #[arg(short, long)]
        campaign: String,
        /// Encounter id
        #[arg(short, long)]
        encounter: i32,
        /// Participant id
        #[arg(long)]
        id: i32,
        /// Amount: positive = damage, negative = healing
        #[arg(long, allow_hyphen_values = true)]
        delta: i32,
    },
    /// Show encounter status (participants, HP, initiative order)
    Show {
        /// Campaign name (e.g. "My Campaign")
        #[arg(short, long)]
        campaign: String,
        /// Encounter id
        #[arg(short, long)]
        encounter: i32,
    },
}

pub fn handle(cmd: EncounterCommands) {
    match cmd {
        EncounterCommands::New { name, campaign } => {
            let dir = find_campaign_dir(&campaign)
                .unwrap_or_else(|| panic!("No campaign named '{}'", campaign));
            let c: Campaign = load(&campaign_file(&dir)).expect("Failed to load campaign");
            // Determine next encounter id by scanning existing encounter files
            let next_id: i32 = next_encounter_id(&dir);
            let enc = Encounter::new(next_id, name.clone(), c.id());
            let path = encounter_file(&dir, next_id);
            save(&enc, &path).expect("Failed to save encounter");
            println!(
                "Saved encounter '{}' (id={}) to {}",
                name,
                next_id,
                path.display()
            );
        }

        EncounterCommands::AddParticipant {
            campaign,
            encounter,
            id,
            kind,
            max_hp,
        } => {
            let dir = find_campaign_dir(&campaign)
                .unwrap_or_else(|| panic!("No campaign named '{}'", campaign));
            let players: PlayerRoster = load(&players_file(&dir)).unwrap_or_default();
            let entities: EntityRoster = load(&entities_file(&dir)).unwrap_or_default();
            let enc_path = encounter_file(&dir, encounter);
            let mut enc: Encounter = load(&enc_path).expect("Failed to load encounter");
            let participant_id = enc.participants().len() as i32 + 1;
            let participant = match kind.as_str() {
                "player" => {
                    let player = players
                        .get(id)
                        .unwrap_or_else(|| panic!("No player with id={}", id));
                    let hp = max_hp.unwrap_or(0);
                    Participant::from_player(participant_id, player, hp)
                }
                _ => {
                    let entity = entities
                        .get(id)
                        .unwrap_or_else(|| panic!("No entity with id={}", id));
                    Participant::from_entity(participant_id, entity, max_hp)
                }
            };
            let name = resolve_name(participant.source(), &players, &entities);
            println!(
                "Added '{}' to encounter '{}' (participant id={})",
                name,
                enc.name(),
                participant.id()
            );
            enc.add_participant(participant);
            save(&enc, &enc_path).expect("Failed to save encounter");
        }

        EncounterCommands::SetInitiative {
            campaign,
            encounter,
            id,
            initiative,
        } => {
            let dir = find_campaign_dir(&campaign)
                .unwrap_or_else(|| panic!("No campaign named '{}'", campaign));
            let enc_path = encounter_file(&dir, encounter);
            let mut enc: Encounter = load(&enc_path).expect("Failed to load encounter");
            let p = enc
                .participant_mut(id)
                .unwrap_or_else(|| panic!("No participant with id={}", id));
            p.set_initiative(initiative);
            println!("Set initiative {} for participant id={}", initiative, id);
            save(&enc, &enc_path).expect("Failed to save encounter");
        }

        EncounterCommands::HpChange {
            campaign,
            encounter,
            id,
            delta,
        } => {
            let dir = find_campaign_dir(&campaign)
                .unwrap_or_else(|| panic!("No campaign named '{}'", campaign));
            let enc_path = encounter_file(&dir, encounter);
            let mut enc: Encounter = load(&enc_path).expect("Failed to load encounter");
            let p = enc
                .participant_mut(id)
                .unwrap_or_else(|| panic!("No participant with id={}", id));
            p.apply_hp_change(delta);
            let action = if delta >= 0 {
                format!("{} damage", delta)
            } else {
                format!("{} healing", delta.abs())
            };
            println!(
                "Participant id={}: {} applied, now {}/{} hp{}",
                id,
                action,
                p.current_hp(),
                p.max_hp(),
                if !p.is_alive() { " [DEAD]" } else { "" }
            );
            save(&enc, &enc_path).expect("Failed to save encounter");
        }

        EncounterCommands::Show {
            campaign,
            encounter,
        } => {
            let dir = find_campaign_dir(&campaign)
                .unwrap_or_else(|| panic!("No campaign named '{}'", campaign));
            let players: PlayerRoster = load(&players_file(&dir)).unwrap_or_default();
            let entities: EntityRoster = load(&entities_file(&dir)).unwrap_or_default();
            let enc: Encounter =
                load(&encounter_file(&dir, encounter)).expect("Failed to load encounter");
            println!("Encounter: {}", enc.name());
            println!("Initiative order:");
            for p in enc.initiative_order() {
                let name = resolve_name(p.source(), &players, &entities);
                let init = p
                    .initiative()
                    .map(|i| i.to_string())
                    .unwrap_or("-".to_string());
                let alive = if p.is_alive() { "" } else { " [DEAD]" };
                println!(
                    "  [{}] {} | init: {} | hp: {}/{}{}",
                    p.id(),
                    name,
                    init,
                    p.current_hp(),
                    p.max_hp(),
                    alive
                );
            }
        }
    }
}

fn next_encounter_id(dir: &std::path::PathBuf) -> i32 {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return 0;
    };
    let mut max_id: Option<i32> = None;
    for entry in entries.flatten() {
        let fname = entry.file_name();
        let fname = fname.to_string_lossy();
        if let Some(rest) = fname.strip_prefix("encounter-")
            && let Some(id_str) = rest.strip_suffix(".ron")
            && let Ok(id) = id_str.parse::<i32>()
        {
            max_id = Some(max_id.map_or(id, |m: i32| m.max(id)));
        }
    }
    max_id.map_or(0, |m| m + 1)
}
