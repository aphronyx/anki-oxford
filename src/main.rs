mod cli;

use anyhow::Result;
use clap::Parser as _;
use cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let res = reqwest::get(cli.oxford().url()).await?;

    Ok(())
}
