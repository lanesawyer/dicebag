mod commands;
mod util;

use clap::Parser;
use commands::Commands;

/// Dicebag's CLI interface
#[derive(Parser)]
#[command(name = "Dicebag CLI")]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        commands::handle(command);
    } else {
        // TODO: Start the application in REPL mode
        println!("Start the app!");
    }
}
