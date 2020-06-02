use crate::data::Album;
use crate::data::Song;
use regex::Error;
use regex::Match;
use regex::Regex;
use std::path::Path;

pub fn parse_album<'a>(album_str: &'a str) -> Album {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r#"(.+) - (.+) \(([0-9]{4})\) \[([A-Z]+)\](.*)"#).unwrap();
    }
    let path = Path::new(album_str);

    let cap = RE.captures(path.file_stem().unwrap().to_str().unwrap()).unwrap();

    let helper_fn = |x: Option<Match>| x.map_or(None, |m| Some(m.as_str().to_owned()));

    Album {
        artist: helper_fn(cap.get(1)),
        name: helper_fn(cap.get(2)),
        year: helper_fn(cap.get(3)),
        media_type: helper_fn(cap.get(4)),
        extra: helper_fn(cap.get(5)),
        songs: None,
    }
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

pub fn parse_song<'a>(song_str: &'a str) -> Song {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"(^[0-9]{2}) - (.*)"#).unwrap();
    }

    println!(">>>> Linea recibida {}", song_str);

    let cap = RE.captures(&song_str).unwrap();

    println!("--->>> Cap: {:?}", cap);

    //Song::new(cap.get(1).map_or("", |m| m.as_str()), cap.get(2).as_str());

    let helper_fn = |x: Option<Match>| x.map_or(None, |m| Some(m.as_str().to_owned()));

    Song {
        number: helper_fn(cap.get(1)),
        name: helper_fn(cap.get(2)),
    }
}
