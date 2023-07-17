mod character;
mod enemy;

use scraper::Html;

pub(in crate::extractor) fn parse_html(html: String) -> Html {
    Html::parse_document(&html)
}
