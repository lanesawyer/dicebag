use clap::{Parser, Subcommand};
use std::num::NonZeroU8;

use core::{Campaign, DiceType, Roll};

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
    Campaign {
        /// Name of the campaign
        #[arg(short, long)]
        name: String,

        /// Description for the campaign
        #[arg(short, long)]
        description: String,
    },
    Player {
        #[arg(short, long)]
        name: String,
    },
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

    match &cli.command {
        Some(Commands::Campaign { name, description }) => {
            let campaign = Campaign::new(0, name.clone(), description.clone());
            // TODO: Save the campaign
            println!("{}: {}", campaign.name(), campaign.description());
        }
        Some(Commands::Player { name }) => {
            // TODO: Save the player
            println!("Hello, {}!", name);
        }
        Some(Commands::Roll { dice, number }) => match dice.parse::<DiceType>() {
            Ok(d) => {
                // TODO: Store history somewhere
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
            Err(d) => {
                println!("Invalid dice type: {}", d);
            }
        },
        None => {
            // TODO: Start the application
            println!("Start the app!");
        }
    }
}
