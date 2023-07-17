use super::{parse_html, parse_url};
use crate::url::get_original_image;
use scraper::Selector;

pub struct Enemy {
    pub link: String,
    pub name: String,
}

pub fn get_enemy_portrait(html: String) -> url::Url {
    let html = parse_html(html);
    let selector = Selector::parse("img.pi-image-thumbnail").unwrap();

    let image = parse_url(
        html.select(&selector)
            .next()
            .unwrap()
            .value()
            .attr("src")
            .unwrap(),
    );

    get_original_image(&image).unwrap()
}

pub fn index_enemies(html: String) -> Vec<Enemy> {
    let html = parse_html(html);
    let selector = Selector::parse("a.category-page__member-link").unwrap();

    html.select(&selector)
        .map(|e| Enemy {
            link: e.value().attr("href").unwrap().to_string(),
            name: e.value().attr("title").unwrap().to_string(),
        })
        .collect::<Vec<_>>()
}
