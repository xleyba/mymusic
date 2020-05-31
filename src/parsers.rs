use crate::data::Album;
use regex::Error;
use regex::Regex;
use std::path::Path;

pub fn parse_album<'a>(album_str: &'a str) -> Album {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r#"(.+) - (.+) \(([0-9]{4})\) \[([A-Z]+)\](.*)"#).unwrap();
    }

    let cap = RE.captures(&album_str).unwrap();

    Album::new(cap.get(1), cap.get(2), cap.get(3), cap.get(4), cap.get(5))
}

pub fn parse_albums(albums: &Vec<String>) -> Option<Vec<Album>> {
    if albums.len() > 0usize {
        let mut vec_albums = Vec::new();

        for entry in albums {
            match Path::new(entry).file_name() {
                Some(file) => vec_albums.push(parse_album(file.to_str().unwrap())),
                None => (),
            }
        }

        if vec_albums.len() > 0 {
            return Some(vec_albums);
        }
    }

    None
}
