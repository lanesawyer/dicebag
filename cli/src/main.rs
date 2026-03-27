mod commands;
mod persistence;
mod util;

use clap::Parser;

/// Dicebag's CLI interface
#[derive(Parser)]
#[command(name = "Dicebag CLI")]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<commands::Commands>,
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
