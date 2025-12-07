use anyhow::bail;
use clap::Parser;
use core::str::FromStr;
use url::Url;

#[derive(Parser)]
pub struct Cli {
    oxford: Oxford,
}

#[derive(Clone)]
struct Oxford {
    id: String,
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
