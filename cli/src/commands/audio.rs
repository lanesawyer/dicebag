use clap::Subcommand;
use core::{AudioCatalog, AudioSubject, Campaign};

use crate::persistence::{data_dir, load, save};
use crate::util::{audio_catalog_filename, campaign_filename};

#[derive(Subcommand)]
pub enum AudioCommands {
    /// List all audio recordings for a campaign
    List {
        /// Campaign file (e.g. my-campaign.ron)
        #[arg(short, long)]
        campaign: String,
    },
    /// Delete an audio recording (removes metadata and the audio file)
    Delete {
        /// Campaign file (e.g. my-campaign.ron)
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
            let c: Campaign = load(&campaign_filename(&campaign)).expect("Failed to load campaign");
            let catalog_file = audio_catalog_filename(c.name());
            let catalog: AudioCatalog = load(&catalog_file).unwrap_or_default();
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
            let c: Campaign = load(&campaign_filename(&campaign)).expect("Failed to load campaign");
            let catalog_file = audio_catalog_filename(c.name());
            let mut catalog: AudioCatalog = load(&catalog_file).unwrap_or_default();
            match catalog.remove(id) {
                None => eprintln!("No recording with id={id} in '{}'", c.name()),
                Some(filename) => {
                    save(&catalog, &catalog_file).expect("Failed to save audio catalog");
                    // Best-effort deletion of the audio file itself
                    let audio_path = data_dir().join(&filename);
                    if audio_path.exists() {
                        if let Err(e) = std::fs::remove_file(&audio_path) {
                            eprintln!("Warning: could not delete audio file {filename}: {e}");
                        } else {
                            println!("Deleted audio file {filename}");
                        }
                    }
                    println!("Deleted recording id={id} from '{}'", c.name());
                }
            }
        }
    }
}
