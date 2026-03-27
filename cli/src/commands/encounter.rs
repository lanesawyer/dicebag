use clap::Subcommand;
use core::{Campaign, Encounter, EntityRoster, Participant, PlayerRoster};

use crate::persistence::load;
use crate::persistence::save;
use crate::util::{encounter_filename, entities_filename, players_filename, resolve_name};

#[derive(Subcommand)]
pub enum EncounterCommands {
    /// Create a new encounter for a campaign
    New {
        #[arg(short, long)]
        name: String,
        /// Campaign file (e.g. my-campaign.ron)
        #[arg(short, long)]
        campaign: String,
    },
    /// Add a participant to an encounter (player or entity by id)
    AddParticipant {
        /// Encounter file (e.g. my-encounter.ron)
        #[arg(short, long)]
        encounter: String,
        /// Campaign file (needed to look up player/entity names)
        #[arg(short, long)]
        campaign: String,
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
        /// Encounter file (e.g. my-encounter.ron)
        #[arg(short, long)]
        encounter: String,
        /// Participant id
        #[arg(long)]
        id: i32,
        #[arg(long)]
        initiative: i32,
    },
    /// Apply damage or healing to a participant (positive = damage, negative = healing)
    HpChange {
        /// Encounter file (e.g. my-encounter.ron)
        #[arg(short, long)]
        encounter: String,
        /// Participant id
        #[arg(long)]
        id: i32,
        /// Amount: positive = damage, negative = healing
        #[arg(long, allow_hyphen_values = true)]
        delta: i32,
    },
    /// Show encounter status (participants, HP, initiative order)
    Show {
        /// Encounter file (e.g. my-encounter.ron)
        #[arg(short, long)]
        encounter: String,
        /// Campaign file (needed to resolve names)
        #[arg(short, long)]
        campaign: String,
    },
}

pub fn handle(cmd: EncounterCommands) {
    match cmd {
        EncounterCommands::New { name, campaign } => {
            let c: Campaign = load(&campaign).expect("Failed to load campaign");
            let enc = Encounter::new(0, name.clone(), c.id());
            let filename = encounter_filename(&name);
            save(&enc, &filename).expect("Failed to save encounter");
            println!("Saved encounter '{}' to {}", name, filename);
        }

        EncounterCommands::AddParticipant {
            encounter,
            campaign,
            id,
            kind,
            max_hp,
        } => {
            let c: Campaign = load(&campaign).expect("Failed to load campaign");
            let players: PlayerRoster = load(&players_filename(c.name())).unwrap_or_default();
            let entities: EntityRoster = load(&entities_filename(c.name())).unwrap_or_default();
            let mut enc: Encounter = load(&encounter).expect("Failed to load encounter");
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
            save(&enc, &encounter).expect("Failed to save encounter");
        }

        EncounterCommands::SetInitiative {
            encounter,
            id,
            initiative,
        } => {
            let mut enc: Encounter = load(&encounter).expect("Failed to load encounter");
            let p = enc
                .participant_mut(id)
                .unwrap_or_else(|| panic!("No participant with id={}", id));
            p.set_initiative(initiative);
            println!("Set initiative {} for participant id={}", initiative, id);
            save(&enc, &encounter).expect("Failed to save encounter");
        }

        EncounterCommands::HpChange {
            encounter,
            id,
            delta,
        } => {
            let mut enc: Encounter = load(&encounter).expect("Failed to load encounter");
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
            save(&enc, &encounter).expect("Failed to save encounter");
        }

        EncounterCommands::Show {
            encounter,
            campaign,
        } => {
            let c: Campaign = load(&campaign).expect("Failed to load campaign");
            let players: PlayerRoster = load(&players_filename(c.name())).unwrap_or_default();
            let entities: EntityRoster = load(&entities_filename(c.name())).unwrap_or_default();
            let enc: Encounter = load(&encounter).expect("Failed to load encounter");
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
