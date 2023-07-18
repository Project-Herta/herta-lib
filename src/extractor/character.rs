use scraper::Selector;

use super::parse_html;

pub struct Character {
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
    let html = parse_html(html);
    let selector = Selector::parse("table.article-table>tbody").unwrap();
    let row_selector = Selector::parse("tr").unwrap();
    let name_selector = Selector::parse("td>a").unwrap();
    let rarity_selector = Selector::parse("td>img").unwrap();
    let path_selector = Selector::parse("td>span>a").unwrap();
    let path_image_selector = Selector::parse("img").unwrap();

    let mut res = vec![];
    let table = html.select(&selector).nth(0).unwrap();
    for entry in table.select(&row_selector).skip(1) {
        let link = entry.select(&name_selector).nth(0).unwrap().value();
        let name = link.attr("title").unwrap();
        let link = link.attr("href").unwrap();

        // we skip the trailblazer because
        // its an 'adaptive' character. there
        // will be a special implementation
        // for Steele and Caelus
        if name == "Trailblazer" {
            continue;
        }

        let rarity = entry.select(&rarity_selector).nth(0).unwrap().value();
        let rarity_image = rarity.attr("data-src").unwrap();
        let rarity = rarity.attr("alt").unwrap();

        let path = entry.select(&path_selector).nth(0).unwrap();
        let path_image = path
            .select(&path_image_selector)
            .nth(0)
            .unwrap()
            .value()
            .attr("data-src")
            .unwrap();
        let path = path.value().attr("title").unwrap();

        let ctype = entry.select(&path_selector).nth(0).unwrap();
        let ctype_image = ctype
            .select(&path_image_selector)
            .nth(0)
            .unwrap()
            .value()
            .attr("data-src")
            .unwrap();
        let ctype = ctype.value().attr("title").unwrap();

        res.push(Character {
            link: link.to_string(),
            name: name.to_string(),
            rarity: rarity.to_string(),
            rarity_image: rarity_image.to_string(),
            path: path.to_string(),
            path_image: path_image.to_string(),
            ctype: ctype.to_string(),
            ctype_image: ctype_image.to_string(),
        })
    }

    res
}
