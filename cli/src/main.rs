use clap::{Parser, Subcommand};
use std::num::NonZeroU8;

use core::{
    Campaign, CombatantRef, DiceType, Encounter, Entity, EntityKind, Participant, Persistable,
    Player, Roll,
};

/// Dicebag's CLI interface
#[derive(Parser)]
#[command(name = "Dicebag CLI")]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new campaign and save it to a RON file
    Campaign {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        description: String,
    },
    /// Add a player to an existing campaign
    Player {
        #[arg(short, long)]
        name: String,
        /// Campaign file to add the player to (e.g. my-campaign.ron)
        #[arg(short, long)]
        campaign: String,
    },
    /// Add a GM-controlled entity (enemy or NPC) to an existing campaign
    AddEntity {
        #[arg(short, long)]
        name: String,
        /// enemy or npc
        #[arg(short, long)]
        kind: String,
        #[arg(long)]
        max_hp: i32,
        /// Campaign file (e.g. my-campaign.ron)
        #[arg(short, long)]
        campaign: String,
    },
    /// List all entities in a campaign
    ListEntities {
        /// Campaign file (e.g. my-campaign.ron)
        #[arg(short, long)]
        campaign: String,
    },
    /// Create a new encounter for a campaign
    AddEncounter {
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
    ShowEncounter {
        /// Encounter file (e.g. my-encounter.ron)
        #[arg(short, long)]
        encounter: String,
        /// Campaign file (needed to resolve names)
        #[arg(short, long)]
        campaign: String,
    },
    /// Roll dice
    Roll {
        /// Dice type to roll (d4, d6, d8, d10, d12, d20, d100)
        #[arg(short, long)]
        dice: String,
        /// Number of dice to roll
        #[arg(short, long)]
        number: Option<NonZeroU8>,
    },
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = &cli.command {
        handle_command(command);
    } else {
        // TODO: Start the application in REPL mode
        println!("Start the app!");
    }
}

fn resolve_name<'a>(source: &CombatantRef, campaign: &'a Campaign) -> &'a str {
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

fn encounter_filename(name: &str) -> String {
    format!("{}.ron", name.to_lowercase().replace(' ', "-"))
}

fn handle_command(command: &Commands) {
    match command {
        Commands::Campaign { name, description } => {
            let campaign = Campaign::new(0, name.to_string(), description.to_string());
            let filename = format!("{}.ron", campaign.name().to_lowercase().replace(' ', "-"));
            campaign
                .save_to_ron_file(&filename)
                .expect("Failed to save campaign");
            println!("Saved campaign '{}' to {}", campaign.name(), filename);
        }

        Commands::Player { name, campaign } => {
            let mut c = Campaign::load_from_ron_file(campaign).expect("Failed to load campaign");
            let id = c.players().len() as i32 + 1;
            let player = Player::new(id, name.to_string());
            println!(
                "Added player '{}' (id={}) to '{}'",
                player.name(),
                player.id(),
                c.name()
            );
            c.add_player(player);
            let filename = format!("{}.ron", c.name().to_lowercase().replace(' ', "-"));
            c.save_to_ron_file(&filename)
                .expect("Failed to save campaign");
        }

        Commands::AddEntity {
            name,
            kind,
            max_hp,
            campaign,
        } => {
            let mut c = Campaign::load_from_ron_file(campaign).expect("Failed to load campaign");
            let entity_kind = match kind.as_str() {
                "npc" => EntityKind::AllyNpc,
                _ => EntityKind::Enemy,
            };
            let id = c.entities().len() as i32 + 1;
            let entity = Entity::new(id, name.to_string(), entity_kind, *max_hp);
            println!(
                "Added entity '{}' (id={}, kind={}, max_hp={}) to '{}'",
                entity.name(),
                entity.id(),
                kind,
                max_hp,
                c.name()
            );
            c.add_entity(entity);
            let filename = format!("{}.ron", c.name().to_lowercase().replace(' ', "-"));
            c.save_to_ron_file(&filename)
                .expect("Failed to save campaign");
        }

        Commands::ListEntities { campaign } => {
            let c = Campaign::load_from_ron_file(campaign).expect("Failed to load campaign");
            let entities = c.entities();
            if entities.is_empty() {
                println!("No entities in '{}'", c.name());
            } else {
                println!("Entities in '{}':", c.name());
                for e in entities {
                    let kind = match e.kind() {
                        EntityKind::Enemy => "enemy",
                        EntityKind::AllyNpc => "npc",
                    };
                    println!(
                        "  [{}] {} ({}, {} hp)  notes: {}",
                        e.id(),
                        e.name(),
                        kind,
                        e.max_hp(),
                        if e.notes().is_empty() { "-" } else { e.notes() }
                    );
                }
            }
        }

        Commands::AddEncounter { name, campaign } => {
            let c = Campaign::load_from_ron_file(campaign).expect("Failed to load campaign");
            let enc = Encounter::new(0, name.to_string(), c.id());
            let filename = encounter_filename(name);
            enc.save_to_ron_file(&filename)
                .expect("Failed to save encounter");
            println!("Saved encounter '{}' to {}", name, filename);
        }

        Commands::AddParticipant {
            encounter,
            campaign,
            id,
            kind,
            max_hp,
        } => {
            let c = Campaign::load_from_ron_file(campaign).expect("Failed to load campaign");
            let mut enc =
                Encounter::load_from_ron_file(encounter).expect("Failed to load encounter");
            let participant_id = enc.participants().len() as i32 + 1;
            let participant = match kind.as_str() {
                "player" => {
                    let player = c
                        .player(*id)
                        .unwrap_or_else(|| panic!("No player with id={}", id));
                    let hp = max_hp.unwrap_or(0); // players need explicit HP since Player has no max_hp yet
                    Participant::from_player(participant_id, player, hp)
                }
                _ => {
                    let entity = c
                        .entity(*id)
                        .unwrap_or_else(|| panic!("No entity with id={}", id));
                    Participant::from_entity(participant_id, entity, *max_hp)
                }
            };
            let name = resolve_name(participant.source(), &c);
            println!(
                "Added '{}' to encounter '{}' (participant id={})",
                name,
                enc.name(),
                participant.id()
            );
            enc.add_participant(participant);
            enc.save_to_ron_file(encounter)
                .expect("Failed to save encounter");
        }

        Commands::SetInitiative {
            encounter,
            id,
            initiative,
        } => {
            let mut enc =
                Encounter::load_from_ron_file(encounter).expect("Failed to load encounter");
            let p = enc
                .participant_mut(*id)
                .unwrap_or_else(|| panic!("No participant with id={}", id));
            p.set_initiative(*initiative);
            println!("Set initiative {} for participant id={}", initiative, id);
            enc.save_to_ron_file(encounter)
                .expect("Failed to save encounter");
        }

        Commands::HpChange {
            encounter,
            id,
            delta,
        } => {
            let mut enc =
                Encounter::load_from_ron_file(encounter).expect("Failed to load encounter");
            let p = enc
                .participant_mut(*id)
                .unwrap_or_else(|| panic!("No participant with id={}", id));
            p.apply_hp_change(*delta);
            let action = if *delta >= 0 {
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
            enc.save_to_ron_file(encounter)
                .expect("Failed to save encounter");
        }

        Commands::ShowEncounter {
            encounter,
            campaign,
        } => {
            let c = Campaign::load_from_ron_file(campaign).expect("Failed to load campaign");
            let enc = Encounter::load_from_ron_file(encounter).expect("Failed to load encounter");
            println!("Encounter: {}", enc.name());
            println!("Initiative order:");
            for p in enc.initiative_order() {
                let name = resolve_name(p.source(), &c);
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

        Commands::Roll { dice, number } => match dice.parse::<DiceType>() {
            Ok(d) => {
                if number.is_none() {
                    println!("Rolling a single {:?}...", d);
                    let result = Roll::roll_one(d);
                    println!("Result: {}", result);
                } else if let Some(n) = number {
                    let result = Roll::roll(&Roll {
                        number: *n,
                        dice: d,
                    });
                    let total = result.iter().sum::<i64>();
                    println!("Rolling a {:?}, {:?}, total = {}", d, result, total);
                }
            }
            Err(d) => println!("Invalid dice type: {}", d),
        },
    }
}
