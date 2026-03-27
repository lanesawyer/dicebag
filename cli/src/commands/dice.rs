use clap::Subcommand;
use core::{DiceType, Roll};
use std::num::NonZeroU8;

#[derive(Subcommand)]
pub enum DiceCommands {
    /// Roll one or more dice
    Roll {
        /// Dice type to roll (d4, d6, d8, d10, d12, d20, d100)
        #[arg(short, long)]
        dice: String,
        /// Number of dice to roll
        #[arg(short, long)]
        number: Option<NonZeroU8>,
    },
}

pub fn handle(cmd: DiceCommands) {
    match cmd {
        DiceCommands::Roll { dice, number } => match dice.parse::<DiceType>() {
            Ok(d) => {
                if let Some(n) = number {
                    let result = Roll::roll(&Roll { number: n, dice: d });
                    let total = result.iter().sum::<i64>();
                    println!("Rolling a {:?}, {:?}, total = {}", d, result, total);
                } else {
                    println!("Rolling a single {:?}...", d);
                    let result = Roll::roll_one(d);
                    println!("Result: {}", result);
                }
            }
            Err(d) => println!("Invalid dice type: {}", d),
        },
    }
}
