mod interface;
mod migrate;
mod reader;

use clap::builder::TypedValueParser;
use clap::{Args, Parser, Subcommand};
use log::*;
use migrate::migrate;
use reader::read as wicked_read;
use std::process::{ExitCode, Termination};

#[derive(Parser)]
#[command(name = "migrate-wicked", version, about, long_about = None)]
struct Cli {
    #[clap(flatten)]
    global_opts: GlobalOpts,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Args)]
struct GlobalOpts {
    #[arg(long, global = true, default_value_t = LevelFilter::Error, value_parser = clap::builder::PossibleValuesParser::new(["TRACE", "DEBUG", "INFO", "WARN", "ERROR"]).map(|s| s.parse::<LevelFilter>().unwrap()),)]
    pub log_level: LevelFilter,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Shows the current xml wicked configuration
    Show {
        /// Format output
        #[arg(value_enum, short, long, default_value_t = Format::Json)]
        format: Format,

        /// Path to file or directory where the wicked xml config is located
        path: String,
    },
    /// Migrate wicked state at path
    Migrate {
        /// Path to file or directory where the wicked xml config is located
        path: String,
    },
}

/// Supported output formats
#[derive(clap::ValueEnum, Clone)]
pub enum Format {
    Json,
    PrettyJson,
    Yaml,
    Text,
}

async fn run_command(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Commands::Show { path, format } => {
            let interfaces = wicked_read(path.into()).await?;
            let output: String = match format {
                Format::Json => serde_json::to_string(&interfaces)?,
                Format::PrettyJson => serde_json::to_string_pretty(&interfaces)?,
                Format::Yaml => serde_yaml::to_string(&interfaces)?,
                Format::Text => format!("{:?}", interfaces),
            };
            println!("{}", output);
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
    let cli = Cli::parse();

    simplelog::TermLogger::init(
        cli.global_opts.log_level,
        simplelog::Config::default(),
        simplelog::TerminalMode::Stderr,
        simplelog::ColorChoice::Auto,
    )
    .unwrap();

    if let Err(error) = run_command(cli).await {
        eprintln!("{:?}", error);
        return CliResult::Error;
    }
    CliResult::Ok
}
