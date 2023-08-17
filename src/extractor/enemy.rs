use super::{parse_html, parse_url};
use crate::url::{canonicalize, get_original_image};
use scraper::{Html, Selector};

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
        .filter_map(|e| {
            let out = Enemy {
                link: canonicalize(e.value().attr("href").unwrap().to_string()),
                name: e.value().attr("title").unwrap().to_string(),
            };

            if out.name.starts_with("Category") {
                None
            } else {
                Some(out)
            }
        })
        .collect()
}

// pub fn get_enemy_drops(html: String, enemy: &mut Enemy) {
//     let html = parse_html(html);
//     let selector = Selector::parse("div.card-container span.card-image a").unwrap();
//     let image_selector = Selector::parse("img").unwrap();

//     let drops = html
//         .select(&selector)
//         .map(|e| {
//             let image = e
//                 .select(&image_selector)
//                 .next()
//                 .unwrap()
//                 .value()
//                 .attr("src")
//                 .unwrap()
//                 .to_string();
//             let link = canonicalize(e.value().attr("href").unwrap().to_string());

//             EnemyDrops { image, link }
//         })
//         .collect::<Vec<_>>();

//     enemy.drops = Some(drops);
// }

pub fn get_enemy_resistances(html: String) -> Vec<u8> {
    let html = parse_html(html);
    let table_selector = Selector::parse("table.wikitable").unwrap();
    let row_selector = Selector::parse("tr").unwrap();
    let col_selector = Selector::parse("td").unwrap();

    select_table(0, html, &table_selector, &row_selector, &col_selector)
}

pub fn get_enemy_debuff_resistances(html: String) -> Vec<u8> {
    let html = parse_html(html);
    let table_selector = Selector::parse("table.wikitable").unwrap();
    let row_selector = Selector::parse("tr").unwrap();
    let col_selector = Selector::parse("td").unwrap();

    select_table(1, html, &table_selector, &row_selector, &col_selector)
}

fn select_table(
    n: usize,
    html: Html,
    table_selector: &Selector,
    row_selector: &Selector,
    col_selector: &Selector,
) -> Vec<u8> {
    html.select(&table_selector)
        .nth(n)
        .unwrap()
        .select(&row_selector)
        .nth(2)
        .unwrap()
        .select(&col_selector)
        .map(|e| {
            e.inner_html()
                .strip_suffix("%")
                .unwrap()
                .parse::<u8>()
                .unwrap()
        })
        .collect()
}
