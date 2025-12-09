mod cli;
mod selector;

use anyhow::{Result, anyhow};
use clap::Parser as _;
use cli::Cli;
use scraper::{Html, Selector};
use selector::ValidSelector as _;
use tokio::{fs, spawn};
use url::Url;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let res = reqwest::get(cli.oxford().url()).await?.text().await?;
    let page = Html::parse_document(&res);
    let pronunciation = page
        .select(&Selector::from_static("div.phons_br"))
        .next()
        .ok_or_else(|| anyhow!("no British pronunciation"))?;
    let audio_url = pronunciation
        .select(&Selector::from_static("div.sound"))
        .next()
        .ok_or_else(|| anyhow!("no audio"))?
        .attr("data-src-mp3")
        .ok_or_else(|| anyhow!("no mp3 audio"))?
        .parse::<Url>()?;
    let audio_file = spawn(async move {
        let audio = spawn(reqwest::get(audio_url).await?.bytes());
        let file_name = format!("{}.mp3", cli.oxford().id());
        fs::write(cli.path().join(&file_name), audio.await??).await?;

        anyhow::Ok(file_name)
    });

    for text in pronunciation
        .select(&Selector::from_static("span.phon"))
        .next()
        .ok_or_else(|| anyhow!("no phonetic"))?
        .text()
    {
        print!("{text}");
    }

    print!("[sound:{}]", audio_file.await??);

    Ok(())
}
