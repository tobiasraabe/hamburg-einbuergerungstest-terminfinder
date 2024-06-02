use clap::{Parser, Subcommand};

pub mod search;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Search for appointments.
    Search { name: Option<String> },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Search { name: _ }) => search::search(),
        #[allow(clippy::match_same_arms)]
        None => search::search(),
    }
}
