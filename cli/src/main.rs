mod persistence;

use clap::{Parser, Subcommand};
use std::num::NonZeroU8;

use core::{
    Campaign, CombatantRef, DiceType, Encounter, Entity, EntityKind, EntityRoster, Participant,
    Player, PlayerRoster, Roll,
};
use persistence::{data_dir, load, save};

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
        /// Campaign file (e.g. my-campaign.ron)
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

fn players_filename(campaign_name: &str) -> String {
    format!(
        "{}-players.ron",
        campaign_name.to_lowercase().replace(' ', "-")
    )
}

fn entities_filename(campaign_name: &str) -> String {
    format!(
        "{}-entities.ron",
        campaign_name.to_lowercase().replace(' ', "-")
    )
}

fn encounter_filename(name: &str) -> String {
    format!("{}.ron", name.to_lowercase().replace(' ', "-"))
}

fn campaign_filename(name: &str) -> String {
    format!("{}.ron", name.to_lowercase().replace(' ', "-"))
}

fn resolve_name<'a>(
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

fn handle_command(command: &Commands) {
    match command {
        Commands::Campaign { name, description } => {
            let campaign = Campaign::new(0, name.to_string(), description.to_string());
            let filename = campaign_filename(campaign.name());
            save(&campaign, &filename).expect("Failed to save campaign");
            // Initialize empty rosters
            save(&PlayerRoster::new(), &players_filename(campaign.name()))
                .expect("Failed to save player roster");
            save(&EntityRoster::new(), &entities_filename(campaign.name()))
                .expect("Failed to save entity roster");
            println!(
                "Saved campaign '{}' to {}",
                campaign.name(),
                data_dir().join(&filename).display()
            );
        }

        Commands::Player { name, campaign } => {
            let c: Campaign = load(campaign).expect("Failed to load campaign");
            let roster_file = players_filename(c.name());
            let mut roster: PlayerRoster = load(&roster_file).unwrap_or_default();
            let id = roster.next_id();
            let player = Player::new(id, name.to_string());
            println!(
                "Added player '{}' (id={}) to '{}'",
                player.name(),
                player.id(),
                c.name()
            );
            roster.add(player);
            save(&roster, &roster_file).expect("Failed to save player roster");
        }

        Commands::AddEntity {
            name,
            kind,
            max_hp,
            campaign,
        } => {
            let c: Campaign = load(campaign).expect("Failed to load campaign");
            let roster_file = entities_filename(c.name());
            let mut roster: EntityRoster = load(&roster_file).unwrap_or_default();
            let entity_kind = match kind.as_str() {
                "npc" => EntityKind::AllyNpc,
                _ => EntityKind::Enemy,
            };
            let id = roster.next_id();
            let entity = Entity::new(id, name.to_string(), entity_kind, *max_hp);
            println!(
                "Added entity '{}' (id={}, kind={}, max_hp={}) to '{}'",
                entity.name(),
                entity.id(),
                kind,
                max_hp,
                c.name()
            );
            roster.add(entity);
            save(&roster, &roster_file).expect("Failed to save entity roster");
        }

        Commands::ListEntities { campaign } => {
            let c: Campaign = load(campaign).expect("Failed to load campaign");
            let roster: EntityRoster = load(&entities_filename(c.name())).unwrap_or_default();
            let entities = roster.entities();
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
            let c: Campaign = load(campaign).expect("Failed to load campaign");
            let enc = Encounter::new(0, name.to_string(), c.id());
            let filename = encounter_filename(name);
            save(&enc, &filename).expect("Failed to save encounter");
            println!(
                "Saved encounter '{}' to {}",
                name,
                data_dir().join(&filename).display()
            );
        }

        Commands::AddParticipant {
            encounter,
            campaign,
            id,
            kind,
            max_hp,
        } => {
            let c: Campaign = load(campaign).expect("Failed to load campaign");
            let players: PlayerRoster = load(&players_filename(c.name())).unwrap_or_default();
            let entities: EntityRoster = load(&entities_filename(c.name())).unwrap_or_default();
            let mut enc: Encounter = load(encounter).expect("Failed to load encounter");
            let participant_id = enc.participants().len() as i32 + 1;
            let participant = match kind.as_str() {
                "player" => {
                    let player = players
                        .get(*id)
                        .unwrap_or_else(|| panic!("No player with id={}", id));
                    Participant::from_player(participant_id, player, max_hp.unwrap_or(0))
                }
                _ => {
                    let entity = entities
                        .get(*id)
                        .unwrap_or_else(|| panic!("No entity with id={}", id));
                    Participant::from_entity(participant_id, entity, *max_hp)
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
            save(&enc, encounter).expect("Failed to save encounter");
        }

        Commands::SetInitiative {
            encounter,
            id,
            initiative,
        } => {
            let mut enc: Encounter = load(encounter).expect("Failed to load encounter");
            let p = enc
                .participant_mut(*id)
                .unwrap_or_else(|| panic!("No participant with id={}", id));
            p.set_initiative(*initiative);
            println!("Set initiative {} for participant id={}", initiative, id);
            save(&enc, encounter).expect("Failed to save encounter");
        }

        Commands::HpChange {
            encounter,
            id,
            delta,
        } => {
            let mut enc: Encounter = load(encounter).expect("Failed to load encounter");
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
            save(&enc, encounter).expect("Failed to save encounter");
        }

        Commands::ShowEncounter {
            encounter,
            campaign,
        } => {
            let c: Campaign = load(campaign).expect("Failed to load campaign");
            let players: PlayerRoster = load(&players_filename(c.name())).unwrap_or_default();
            let entities: EntityRoster = load(&entities_filename(c.name())).unwrap_or_default();
            let enc: Encounter = load(encounter).expect("Failed to load encounter");
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
