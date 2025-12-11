mod cli;
mod selector;

use anyhow::{Result, anyhow};
use clap::Parser as _;
use cli::Cli;
use scraper::{Html, Selector};
use selector::ValidSelector as _;
use std::{
    env::set_current_dir,
    fs::OpenOptions,
    io::{Write, stdout},
};
use tokio::{fs, spawn};
use url::{Url, form_urlencoded};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    set_current_dir(cli.path())?;

    let res = reqwest::get(cli.oxford().url()).await?.text().await?;
    let page = Html::parse_document(&res);
    let british_pronunciation = page
        .select(&Selector::from_static("div.phons_br"))
        .next()
        .ok_or_else(|| anyhow!("no British pronunciation"))?;
    let audio_url = british_pronunciation
        .select(&Selector::from_static("div.sound"))
        .next()
        .ok_or_else(|| anyhow!("no audio"))?
        .attr("data-src-mp3")
        .ok_or_else(|| anyhow!("no mp3 audio"))?
        .parse::<Url>()?;
    let id = cli.oxford().id().to_owned();
    let audio_file = spawn(async move {
        let audio = spawn(reqwest::get(audio_url).await?.bytes());
        let file_name = format!("{id}.mp3");
        fs::write(&file_name, audio.await??).await?;

        anyhow::Ok(file_name)
    });

    let word = page
        .select(&Selector::from_static("h1"))
        .next()
        .ok_or_else(|| anyhow!("no word"))?
        .text()
        .collect();

    let part_of_speech = page
        .select(&Selector::from_static("span.pos"))
        .next()
        .ok_or_else(|| anyhow!("no part of speech"))?
        .text()
        .collect();

    let definitions = page
        .select(&Selector::from_static("span.def"))
        .map(|span| span.text().collect())
        .collect::<Vec<String>>();
    let is_polysemous = definitions.len() > 1;

    let mut pronunciation = british_pronunciation
        .select(&Selector::from_static("span.phon"))
        .next()
        .ok_or_else(|| anyhow!("no phonetic"))?
        .text()
        .collect::<String>();
    pronunciation.push_str("[sound:");
    pronunciation.push_str(&audio_file.await??);
    pronunciation.push(']');

    let wtr: Box<dyn Write> = if cli.output() {
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("anki-oxford.csv")?;
        Box::new(file)
    } else {
        Box::new(stdout())
    };
    let mut csv = csv::Writer::from_writer(wtr);
    for definition in definitions {
        let mut dictionary = cli.oxford().id().to_owned();
        if is_polysemous {
            dictionary.push_str("#:~:text=");
            for str in form_urlencoded::byte_serialize(definition.as_bytes()) {
                dictionary.push_str(if str == "+" { "%20" } else { str });
            }
        }

        csv.write_record([
            &word,
            &pronunciation,
            &part_of_speech,
            &definition,
            &dictionary,
        ])?;
    }

    Ok(())
}
