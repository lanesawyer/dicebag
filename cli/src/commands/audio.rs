use clap::Subcommand;
use core::{AudioCatalog, AudioSubject, Campaign};

use crate::persistence::{find_campaign_dir, load, save};
use crate::util::{audio_catalog_file, audio_file, campaign_file};

#[derive(Subcommand)]
pub enum AudioCommands {
    /// List all audio recordings for a campaign
    List {
        /// Campaign name (e.g. "My Campaign")
        #[arg(short, long)]
        campaign: String,
    },
    /// Delete an audio recording (removes metadata and the audio file)
    Delete {
        /// Campaign name (e.g. "My Campaign")
        #[arg(short, long)]
        campaign: String,
        /// ID of the recording to delete
        #[arg(short, long)]
        id: i32,
    },
}

pub fn handle(cmd: AudioCommands) {
    match cmd {
        AudioCommands::List { campaign } => {
            let dir = find_campaign_dir(&campaign)
                .unwrap_or_else(|| panic!("No campaign named '{}'", campaign));
            let c: Campaign = load(&campaign_file(&dir)).expect("Failed to load campaign");
            let catalog: AudioCatalog = load(&audio_catalog_file(&dir)).unwrap_or_default();
            let recordings = catalog.recordings();
            if recordings.is_empty() {
                println!("No audio recordings in '{}'", c.name());
            } else {
                println!("Audio recordings in '{}':", c.name());
                for r in recordings {
                    let subject = match r.subject() {
                        AudioSubject::Player(id) => format!("player {id}"),
                        AudioSubject::Entity(id) => format!("entity {id}"),
                    };
                    println!(
                        "  [{}] {} — {} — {}{}",
                        r.id(),
                        r.label(),
                        subject,
                        r.filename(),
                        if r.notes().is_empty() {
                            String::new()
                        } else {
                            format!("  ({})", r.notes())
                        }
                    );
                }
            }
        }

        AudioCommands::Delete { campaign, id } => {
            let dir = find_campaign_dir(&campaign)
                .unwrap_or_else(|| panic!("No campaign named '{}'", campaign));
            let c: Campaign = load(&campaign_file(&dir)).expect("Failed to load campaign");
            let mut catalog: AudioCatalog = load(&audio_catalog_file(&dir)).unwrap_or_default();
            match catalog.remove(id) {
                None => eprintln!("No recording with id={id} in '{}'", c.name()),
                Some(_filename) => {
                    save(&catalog, &audio_catalog_file(&dir))
                        .expect("Failed to save audio catalog");
                    let audio_path = audio_file(&dir, id);
                    if audio_path.exists() {
                        if let Err(e) = std::fs::remove_file(&audio_path) {
                            eprintln!("Warning: could not delete audio file: {e}");
                        } else {
                            println!("Deleted audio file {}", audio_path.display());
                        }
                    }
                    println!("Deleted recording id={id} from '{}'", c.name());
                }
            }
        }
    }
}
