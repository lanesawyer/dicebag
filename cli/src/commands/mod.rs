pub mod audio;
pub mod campaign;
pub mod dice;
pub mod encounter;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// Campaign and roster management
    #[command(subcommand)]
    Campaign(campaign::CampaignCommands),
    /// Encounter tracking
    #[command(subcommand)]
    Encounter(encounter::EncounterCommands),
    /// Dice rolling
    #[command(subcommand)]
    Dice(dice::DiceCommands),
    /// Audio recording management
    #[command(subcommand)]
    Audio(audio::AudioCommands),
}

pub fn handle(command: Commands) {
    match command {
        Commands::Campaign(cmd) => campaign::handle(cmd),
        Commands::Encounter(cmd) => encounter::handle(cmd),
        Commands::Dice(cmd) => dice::handle(cmd),
        Commands::Audio(cmd) => audio::handle(cmd),
    }
}
