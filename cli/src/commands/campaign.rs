use clap::Subcommand;
use core::{Campaign, Entity, EntityKind, EntityRoster, Player, PlayerRoster};

use crate::persistence::{campaign_dir, find_campaign_dir, load, save};
use crate::util::{campaign_file, entities_file, players_file};

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
        /// Campaign name (e.g. "My Campaign")
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
        /// Campaign name (e.g. "My Campaign")
        #[arg(short, long)]
        campaign: String,
    },
    /// List all entities in a campaign
    ListEntities {
        /// Campaign name (e.g. "My Campaign")
        #[arg(short, long)]
        campaign: String,
    },
}

pub fn handle(cmd: CampaignCommands) {
    match cmd {
        CampaignCommands::New { name, description } => {
            // Count existing campaigns to assign a sequential id
            let existing = crate::persistence::list_campaign_dirs().len() as i32;
            let campaign = Campaign::new(existing, name.clone(), description);
            let dir = campaign_dir(existing, &name);
            save(&campaign, &campaign_file(&dir)).expect("Failed to save campaign");
            save(&PlayerRoster::new(), &players_file(&dir)).expect("Failed to save player roster");
            save(&EntityRoster::new(), &entities_file(&dir)).expect("Failed to save entity roster");
            println!("Saved campaign '{}' to {}", campaign.name(), dir.display());
        }

        CampaignCommands::AddPlayer { name, campaign } => {
            let dir = find_campaign_dir(&campaign)
                .unwrap_or_else(|| panic!("No campaign named '{}'", campaign));
            let c: Campaign = load(&campaign_file(&dir)).expect("Failed to load campaign");
            let mut roster: PlayerRoster = load(&players_file(&dir)).unwrap_or_default();
            let id = roster.next_id();
            let player = Player::new(id, name);
            println!(
                "Added player '{}' (id={}) to '{}'",
                player.name(),
                player.id(),
                c.name()
            );
            roster.add(player);
            save(&roster, &players_file(&dir)).expect("Failed to save player roster");
        }

        CampaignCommands::AddEntity {
            name,
            kind,
            max_hp,
            campaign,
        } => {
            let dir = find_campaign_dir(&campaign)
                .unwrap_or_else(|| panic!("No campaign named '{}'", campaign));
            let c: Campaign = load(&campaign_file(&dir)).expect("Failed to load campaign");
            let mut roster: EntityRoster = load(&entities_file(&dir)).unwrap_or_default();
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
            save(&roster, &entities_file(&dir)).expect("Failed to save entity roster");
        }

        CampaignCommands::ListEntities { campaign } => {
            let dir = find_campaign_dir(&campaign)
                .unwrap_or_else(|| panic!("No campaign named '{}'", campaign));
            let c: Campaign = load(&campaign_file(&dir)).expect("Failed to load campaign");
            let roster: EntityRoster = load(&entities_file(&dir)).unwrap_or_default();
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
