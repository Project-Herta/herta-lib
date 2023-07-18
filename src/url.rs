use std::fmt::Display;

use url::Url;

#[derive(Debug)]
pub enum UrlError<'a> {
    NoBaseUrl(&'a Url),
}

pub(crate) fn get_original_image<'a>(url: &'a Url) -> Result<Url, UrlError<'a>> {
    let cb = url
        .query_pairs()
        .filter(|(key, _val)| key.contains("cb"))
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<String>();

    let segments = url
        .path_segments()
        .ok_or_else(|| UrlError::NoBaseUrl(url))?
        .map_while(|seg| {
            if seg.contains("scale-to-width") {
                return None;
            }

            Some(seg)
        })
        .collect::<Vec<_>>()
        .join("/");

    let base = format!(
        "{}://{}/{}",
        url.scheme(),
        url.host_str().unwrap(),
        segments
    );
    let mut res = Url::parse(&base).unwrap();

    res.set_query(Some(cb.as_str()));

    Ok(res)
}

pub(crate) fn canonicalize<U: Display>(url: U) -> String {
    format!("https://honkai-star-rail.fandom.com{}", url)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_original_image_works() {
        let raw = "https://static.wikia.nocookie.net/houkai-star-rail/images/c/c2/Character_Clara_Splash_Art.png/revision/latest/scale-to-width-down/185?cb=20230216231958";
        let expected = "https://static.wikia.nocookie.net/houkai-star-rail/images/c/c2/Character_Clara_Splash_Art.png/revision/latest?cb=20230216231958";

        let url = Url::parse(raw).unwrap();
        let expected_url = Url::parse(expected).unwrap();

        let original_image = get_original_image(&url);

        assert!(original_image.is_ok());
        assert_eq!(original_image.unwrap(), expected_url);
    }
}
