use clap::{Parser, Subcommand};

pub mod search;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Search for appointments.
    Search,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Search => search::search(),
    }
}
