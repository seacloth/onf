mod config;
mod profile;

use clap::{Parser, Subcommand};
use colored::Colorize;

#[derive(Parser)]
#[command(name = "onf", about = "Minimal config profile switcher", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    New { name: String },
    Apply { name: String },
    List,
    Status,
    Delete { name: String },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::New { name } => profile::create(&name),
        Commands::Apply { name } => profile::apply(&name),
        Commands::List => profile::list(),
        Commands::Status => profile::status(),
        Commands::Delete { name } => profile::delete(&name),
    };

    if let Err(e) = result {
        eprintln!("{} {}", "error:".red().bold(), e);
        std::process::exit(1);
    }
}
