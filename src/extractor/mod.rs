mod character;
mod enemy;

pub use character::*;
pub use enemy::*;
use scraper::Html;
use url::Url;

pub(in crate::extractor) fn parse_html(html: String) -> Html {
    Html::parse_document(&html)
}

pub(in crate::extractor) fn parse_url(url: &str) -> Url {
    Url::parse(url).unwrap()
}
