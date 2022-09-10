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
            Commands::New { id } => Commands::run_new(&id).await?,
            Commands::Test { id } => Commands::run_test(&id)?,
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
    Test {
        #[clap(value_parser)]
        id: String,
    },
}

impl Commands {
    async fn run_new(id: &str) -> Result<()> {
        Problem::create(id).await?;
        Ok(())
    }
    fn run_test(id: &str) -> Result<()> {
        let p = Problem::read(id)?;
        p.run_examples()?;
        Ok(())
    }
}
