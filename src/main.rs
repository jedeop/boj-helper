use anyhow::Result;
use boj_helper::cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    Cli::run().await?;

    Ok(())
}
