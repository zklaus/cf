use std::process;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about = "A c-forge CLI tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Feedstock {
        #[command(subcommand)]
        command: FeedstockCommands,
    },
    StagedRecipes,
    Status,
}

#[derive(Subcommand, Debug)]
enum FeedstockCommands {
    Setup,
    Work,
    ListPrs,
    Rerender,
    UpdateVersion,
    Lint,
    Migrator,
    Config,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Feedstock { command } => match command {
            FeedstockCommands::Setup => {
                eprintln!("Command not implemented yet.");
                process::exit(1);
            }
            FeedstockCommands::Work => {
                eprintln!("Command not implemented yet.");
                process::exit(1);
            }
            FeedstockCommands::ListPrs => {
                eprintln!("Command not implemented yet.");
                process::exit(1);
            }
            FeedstockCommands::Rerender => {
                eprintln!("Command not implemented yet.");
                process::exit(1);
            }
            FeedstockCommands::UpdateVersion => {
                eprintln!("Command not implemented yet.");
                process::exit(1);
            }
            FeedstockCommands::Lint => {
                eprintln!("Command not implemented yet.");
                process::exit(1);
            }
            FeedstockCommands::Migrator => {
                eprintln!("Command not implemented yet.");
                process::exit(1);
            }
            FeedstockCommands::Config => {
                eprintln!("Command not implemented yet.");
                process::exit(1);
            }
        },
        Commands::StagedRecipes => {
            eprintln!("Staged recipe commands are not yet implemented.");
            process::exit(1);
        }
        Commands::Status => {
            eprintln!("Status commands are not yet implemented.");
            process::exit(1);
        }
    }
}