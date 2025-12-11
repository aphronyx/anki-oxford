use anyhow::bail;
use clap::Parser;
use core::str::FromStr;
use std::path::{Path, PathBuf};
use url::Url;

#[derive(Parser)]
pub struct Cli {
    oxford: Oxford,
    #[arg(long)]
    path: PathBuf,
    #[arg(short, long)]
    output: bool,
}

impl Cli {
    pub const fn oxford(&self) -> &Oxford {
        &self.oxford
    }

    pub const fn output(&self) -> bool {
        self.output
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

#[derive(Clone)]
pub struct Oxford {
    id: String,
}

impl Oxford {
    pub const fn id(&self) -> &str {
        self.id.as_str()
    }

    pub fn url(&self) -> Url {
        #![expect(clippy::unwrap_used, reason = "valid URL")]
        let mut url =
            Url::parse("https://www.oxfordlearnersdictionaries.com/definition/english").unwrap();
        url.path_segments_mut().unwrap().push(&self.id);
        url
    }
}

impl FromStr for Oxford {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let url = Url::parse(s)?;
        if url.domain() != Some("www.oxfordlearnersdictionaries.com") {
            bail!("not Oxford Learner's Dictionaries URL");
        }

        #[expect(clippy::unwrap_used, reason = "url with domain necessarily has path")]
        let mut path = url.path_segments().unwrap();
        if path.next() == Some("definition")
            && path.next() == Some("english")
            && let Some(id) = path.next().filter(|segment| !segment.is_empty())
        {
            Ok(Self { id: id.into() })
        } else {
            bail!("not word URL");
        }
    }
}
