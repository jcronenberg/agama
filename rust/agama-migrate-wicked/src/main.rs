mod interface;
mod migrate;
mod reader;

use clap::{Parser, Subcommand};
use migrate::migrate;
use reader::read as wicked_read;
use std::process::{ExitCode, Termination};
use log::*;

#[derive(Parser)]
#[command(name = "migrate-wicked", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Shows the current xml wicked configuration
    Show {
        /// Where wicked xml configs are located
        path: String
    },
    /// Migrate wicked state at path
    Migrate {
        /// Where wicked xml configs are located
        path: String
    },
}

async fn run_command(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Commands::Show { path } => {
            let interfaces = wicked_read(path.into()).await?;
            println!("{:?}", interfaces);
            Ok(())
        }
        Commands::Migrate { path } => {
            migrate(path).await.unwrap();
            Ok(())
        }
    }
}

/// Represents the result of execution.
pub enum CliResult {
    /// Successful execution.
    Ok = 0,
    /// Something went wrong.
    Error = 1,
}

impl Termination for CliResult {
    fn report(self) -> ExitCode {
        ExitCode::from(self as u8)
    }
}

#[async_std::main]
async fn main() -> CliResult {
    simplelog::TermLogger::init(
            LevelFilter::Info,
            simplelog::Config::default(),
            simplelog::TerminalMode::Stderr,
            simplelog::ColorChoice::Auto,
        )
        .unwrap();

    let cli = Cli::parse();

    if let Err(error) = run_command(cli).await {
        eprintln!("{:?}", error);
        return CliResult::Error;
    }
    CliResult::Ok
}
