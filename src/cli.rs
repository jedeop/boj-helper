use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::problem::Problem;

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

impl Cli {
    pub async fn run() -> Result<()> {
        let cli = Cli::parse();

        match &cli.command {
            Commands::New { id } => Commands::new(&id).await?,
        };

        Ok(())
    }
}

#[derive(Subcommand)]
enum Commands {
    New {
        #[clap(value_parser)]
        id: String,
    },
}

impl Commands {
    async fn new(id: &str) -> Result<()> {
        Problem::create(id).await?;
        Ok(())
    }
}
