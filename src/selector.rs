use scraper::Selector;

pub trait ValidSelector {
    fn from_static(selectors: &'static str) -> Self;
}

impl ValidSelector for Selector {
    fn from_static(selectors: &'static str) -> Self {
        #[expect(clippy::unwrap_used, reason = "valid selector")]
        Self::parse(selectors).unwrap()
    }
}
