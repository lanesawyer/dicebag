use clap::Subcommand;
use core::{Campaign, Entity, EntityKind, EntityRoster, Player, PlayerRoster};

use crate::persistence::{data_dir, load, save};
use crate::util::{campaign_filename, entities_filename, players_filename};

#[derive(Subcommand)]
pub enum CampaignCommands {
    /// Create a new campaign and save it to a RON file
    New {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        description: String,
    },
    /// Add a player to an existing campaign
    AddPlayer {
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
}

pub fn handle(cmd: CampaignCommands) {
    match cmd {
        CampaignCommands::New { name, description } => {
            let campaign = Campaign::new(0, name, description);
            let filename = campaign_filename(campaign.name());
            save(&campaign, &filename).expect("Failed to save campaign");
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

        CampaignCommands::AddPlayer { name, campaign } => {
            let c: Campaign = load(&campaign).expect("Failed to load campaign");
            let roster_file = players_filename(c.name());
            let mut roster: PlayerRoster = load(&roster_file).unwrap_or_default();
            let id = roster.next_id();
            let player = Player::new(id, name);
            println!(
                "Added player '{}' (id={}) to '{}'",
                player.name(),
                player.id(),
                c.name()
            );
            roster.add(player);
            save(&roster, &roster_file).expect("Failed to save player roster");
        }

        CampaignCommands::AddEntity {
            name,
            kind,
            max_hp,
            campaign,
        } => {
            let c: Campaign = load(&campaign).expect("Failed to load campaign");
            let roster_file = entities_filename(c.name());
            let mut roster: EntityRoster = load(&roster_file).unwrap_or_default();
            let entity_kind = match kind.as_str() {
                "npc" => EntityKind::AllyNpc,
                _ => EntityKind::Enemy,
            };
            let id = roster.next_id();
            let entity = Entity::new(id, name, entity_kind, max_hp);
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

        CampaignCommands::ListEntities { campaign } => {
            let c: Campaign = load(&campaign).expect("Failed to load campaign");
            let roster_file = entities_filename(c.name());
            let roster: EntityRoster = load(&roster_file).unwrap_or_default();
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
    }
}
