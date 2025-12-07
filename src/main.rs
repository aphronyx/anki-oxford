mod cli;
mod selector;

use anyhow::{Result, anyhow};
use clap::Parser as _;
use cli::Cli;
use dirs::download_dir;
use scraper::{Html, Selector};
use selector::ValidSelector as _;
use tokio::fs;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let res = reqwest::get(cli.oxford().url()).await?.text().await?;
    let page = Html::parse_document(&res);
    let audio_url = page
        .select(&Selector::from_static("div.phons_br > div.sound"))
        .next()
        .ok_or_else(|| anyhow!("no audio"))?
        .attr("data-src-mp3")
        .ok_or_else(|| anyhow!("no mp3 audio"))?;
    let audio = reqwest::get(audio_url).await?.bytes().await?;
    #[expect(clippy::unwrap_used, reason = "downloads folder exists")]
    fs::write(
        download_dir()
            .unwrap()
            .join(cli.oxford().id())
            .with_extension("mp3"),
        audio,
    )
    .await?;

    Ok(())
}
