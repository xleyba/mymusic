use regex::Error;
use regex::Regex;

pub fn parse_album<'a>(album_str: &'a str) -> Album {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r#"(.+) - (.+) \(([0-9]{4})\) \[([A-Z]+)\](.*)"#).unwrap();
    }

    let cap = RE.captures(&album_str).unwrap();

    Ok(Album::new(
        cap.get(1),
        cap.get(2),
        cap.get(3),
        cap.get(4),
        cap.get(5),
    ))

}