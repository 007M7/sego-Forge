//! Sego-Forge CLI — the command-line entry point.
//!
//! Run with: `cargo run -- <command>`
use clap::{Parser, Subcommand};
use tracing_subscriber::{fmt, EnvFilter};

mod commands;

#[derive(Parser)]
#[command(name = "forge")]
#[command(version, about = "Sego-Forge: AI-powered development workflow engine")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new forge project
    Init {
        /// Project name
        name: String,
    },
    /// Run the workflow pipeline
    Run {
        /// Workflow configuration file
        #[arg(short, long, default_value = "forge.toml")]
        config: String,
    },
    /// Show workflow status
    Status,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name } => {
            commands::init::run(&name)?;
        }
        Commands::Run { config } => {
            commands::run::execute(&config).await?;
        }
        Commands::Status => {
            commands::status::show()?;
        }
    }

    Ok(())
}
