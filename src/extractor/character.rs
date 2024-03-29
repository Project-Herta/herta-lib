use scraper::Selector;

use super::{parse_html, parse_url};
use crate::url::{canonicalize, get_original_image};

pub struct Character {
    pub id: usize,
    pub name: String,
    pub link: String,
    pub rarity: String,
    pub rarity_image: String,
    pub path: String,
    pub path_image: String,
    pub ctype: String,
    pub ctype_image: String,
}

pub fn index_characters(html: String) -> Vec<Character> {
    let html = parse_html(&html);
    let selector = Selector::parse("table.article-table>tbody").unwrap();
    let row_selector = Selector::parse("tr").unwrap();
    let name_selector = Selector::parse("td>a").unwrap();
    let rarity_selector = Selector::parse("td>img").unwrap();
    let path_selector = Selector::parse("td>span>a").unwrap();
    let path_image_selector = Selector::parse("img").unwrap();
    let ctype_selector = Selector::parse("td>span>span>b").unwrap();
    let ctype_image_selector = Selector::parse("td img").unwrap();

    let mut res = vec![];
    let table = html.select(&selector).next().unwrap();
    for (indx, entry) in table.select(&row_selector).skip(1).into_iter().enumerate() {
        let link = entry.select(&name_selector).next().unwrap().value();
        let name = link.attr("title").unwrap();
        let link = canonicalize(link.attr("href").unwrap());

        // we skip the trailblazer because
        // its an 'adaptive' character. there
        // will be a special implementation
        // for Steele and Caelus
        if name == "Trailblazer" {
            continue;
        }

        let rarity = entry.select(&rarity_selector).next().unwrap().value();
        let rarity_image = rarity.attr("data-src").unwrap();
        let rarity = rarity.attr("alt").unwrap();

        let path = entry.select(&path_selector).next().unwrap();
        let path_image = path
            .select(&path_image_selector)
            .next()
            .unwrap()
            .value()
            .attr("data-src")
            .unwrap();
        let path = path.value().attr("title").unwrap();

        let ctype = entry.select(&ctype_selector).next().unwrap();
        let ctype_image = entry
            .select(&ctype_image_selector)
            .last()
            .unwrap()
            .value()
            .attr("data-src")
            .unwrap();
        let ctype = ctype.inner_html();

        res.push(Character {
            id: indx,
            link: link.to_string(),
            name: name.to_string(),
            rarity: rarity.to_string(),
            rarity_image: get_original_image(&parse_url(rarity_image))
                .unwrap()
                .to_string(),
            path: path.to_string(),
            path_image: path_image.to_string(),
            ctype: ctype.to_string(),
            ctype_image: get_original_image(&parse_url(ctype_image))
                .unwrap()
                .to_string(),
        })
    }

    res
}

pub fn get_character_art(html: String) -> Option<(String, String)> {
    let html = parse_html(&html);
    let portrait_selector = Selector::parse("img[alt=Portrait]").ok()?;
    let splash_selector = Selector::parse("img[alt=\"Splash Art\"]").ok()?;

    let portrait = get_original_image(&parse_url(
        html.select(&portrait_selector)
            .next()?
            .value()
            .attr("data-src")?,
    ))
    .unwrap()
    .to_string();

    let splash = get_original_image(&parse_url(
        html.select(&splash_selector)
            .next()?
            .value()
            .attr("data-src")?,
    ))
    .unwrap()
    .to_string();

    Some((portrait, splash))
}

pub fn get_voice_overs(html: String) -> Vec<(String, String)> {
    let html = parse_html(&html);
    let voice_over_entry = Selector::parse("table.wikitable>tbody>tr").unwrap();
    let vo_type = Selector::parse("th>div").unwrap();
    let vo_audio = Selector::parse("td>span>a").unwrap();

    let mut res = vec![];
    for voice_over in html.select(&voice_over_entry) {
        let audio = voice_over.select(&vo_audio).next();
        let audio_type = voice_over.select(&vo_type).next();

        if audio.is_none() || audio_type.is_none() {
            continue;
        }

        let audio = audio.unwrap();

        if audio
            .value()
            .classes()
            .collect::<Vec<_>>()
            .contains(&"no-audio")
            || audio.value().attr("href").unwrap().starts_with("/")
        {
            continue;
        }

        let audio_link = audio.value().attr("href").unwrap().to_string();
        let audio_type = audio_type.unwrap().value().id().unwrap().to_string();
        // let audio_type = audio_type.value().id().unwrap().to_string();

        res.push((audio_type, audio_link));
    }

    res
}
