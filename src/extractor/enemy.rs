use super::{parse_html, parse_url};
use crate::url::{canonicalize, get_original_image};
use scraper::{Html, Selector};

pub struct Enemy {
    pub link: String,
    pub name: String,
    pub portrait: String,
    pub res: Vec<u8>,
    pub debuff_res: Vec<u8>,
}

fn get_enemy_portrait(html: &Html) -> String {
    let selector = Selector::parse("img.pi-image-thumbnail").unwrap();

    let image = parse_url(
        html.select(&selector)
            .next()
            .unwrap()
            .value()
            .attr("src")
            .unwrap(),
    );

    get_original_image(&image).unwrap().to_string()
}

pub fn index_enemies(html: String) -> Vec<Enemy> {
    let html = parse_html(html);
    let selector = Selector::parse("a.category-page__member-link").unwrap();

    html.select(&selector)
        .filter_map(|e| {
            let res = get_enemy_resistances(&html);
            let debuff_res = get_enemy_debuff_resistances(&html);
            let portrait = get_enemy_portrait(&html);

            let out = Enemy {
                link: canonicalize(e.value().attr("href").unwrap().to_string()),
                name: e.value().attr("title").unwrap().to_string(),
                portrait,
                res,
                debuff_res,
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

fn get_enemy_resistances(html: &Html) -> Vec<u8> {
    let table_selector = Selector::parse("table.wikitable").unwrap();
    let row_selector = Selector::parse("tr").unwrap();
    let col_selector = Selector::parse("td").unwrap();

    select_table(0, html, &table_selector, &row_selector, &col_selector)
}

fn get_enemy_debuff_resistances(html: &Html) -> Vec<u8> {
    let table_selector = Selector::parse("table.wikitable").unwrap();
    let row_selector = Selector::parse("tr").unwrap();
    let col_selector = Selector::parse("td").unwrap();

    select_table(1, html, &table_selector, &row_selector, &col_selector)
}

fn select_table(
    n: usize,
    html: &Html,
    table_selector: &Selector,
    row_selector: &Selector,
    col_selector: &Selector,
) -> Vec<u8> {
    html.select(table_selector)
        .nth(n)
        .unwrap()
        .select(row_selector)
        .nth(2)
        .unwrap()
        .select(col_selector)
        .map(|e| {
            let percent = e.inner_html();

            // This is weird syntax, but
            // it basically strips the newline
            // if it exists
            if percent.ends_with('\n') {
                percent.strip_suffix('\n').unwrap()
            } else {
                percent.as_str()
            }
            .strip_suffix('%')
            .unwrap()
            .parse::<u8>()
            .unwrap()
        })
        .collect()
}
