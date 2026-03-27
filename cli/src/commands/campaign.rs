use clap::Subcommand;
use core::{Campaign, Entity, EntityKind, Persistable, Player};

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
            let filename = format!("{}.ron", campaign.name().to_lowercase().replace(' ', "-"));
            campaign
                .save_to_ron_file(&filename)
                .expect("Failed to save campaign");
            println!("Saved campaign '{}' to {}", campaign.name(), filename);
        }

        CampaignCommands::AddPlayer { name, campaign } => {
            let mut c = Campaign::load_from_ron_file(&campaign).expect("Failed to load campaign");
            let id = c.players().len() as i32 + 1;
            let player = Player::new(id, name);
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

        CampaignCommands::AddEntity {
            name,
            kind,
            max_hp,
            campaign,
        } => {
            let mut c = Campaign::load_from_ron_file(&campaign).expect("Failed to load campaign");
            let entity_kind = match kind.as_str() {
                "npc" => EntityKind::AllyNpc,
                _ => EntityKind::Enemy,
            };
            let id = c.entities().len() as i32 + 1;
            let entity = Entity::new(id, name, entity_kind, max_hp);
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

        CampaignCommands::ListEntities { campaign } => {
            let c = Campaign::load_from_ron_file(&campaign).expect("Failed to load campaign");
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
    }
}
