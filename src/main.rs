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
    Apply {
        name: String,
        #[arg(long)]
        dry_run: bool,
    },
    Edit { name: String },
    List,
    Status,
    Delete { name: String },
    Restore {
        #[arg(long)]
        dry_run: bool,
    },
    Copy { from: String, to: String },
    Export { name: String, path: String },
    Import { path: String },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::New { name } => profile::create(&name),
        Commands::Apply { name, dry_run } => profile::apply(&name, dry_run),
        Commands::Edit { name } => profile::edit(&name),
        Commands::List => profile::list(),
        Commands::Status => profile::status(),
        Commands::Delete { name } => profile::delete(&name),
        Commands::Restore { dry_run } => profile::restore(dry_run),
        Commands::Copy { from, to } => profile::copy(&from, &to),
        Commands::Export { name, path } => profile::export(&name, &path),
        Commands::Import { path } => profile::import(&path),
    };

    if let Err(e) = result {
        eprintln!("{} {}", "error:".red().bold(), e);
        std::process::exit(1);
    }
