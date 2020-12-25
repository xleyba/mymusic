use std::path::Path;

use regex::{Error, Match, Regex};

use crate::data::Album;
use crate::data::Song;
use crate::error::{MyMusicError, Result};

pub fn parse_album<'a>(album_str: &'a str) -> Option<Album> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r#"(.+) - (.+) \(([0-9]{4})\) \[([A-Z]+)\](.*)"#).unwrap();
    }
    let path = Path::new(album_str);

    debug!(":: Parsing album: {}", album_str);

    match path.file_stem() {
        Some(dir) => {
            let cap = RE.captures(dir.to_str().unwrap()).unwrap();

            let helper_fn = |x: Option<Match>|
                x.map_or(None, |m| Some(m.as_str().to_owned()));

            Some(Album {
                artist: helper_fn(cap.get(1)),
                name: helper_fn(cap.get(2)),
                year: helper_fn(cap.get(3)),
                media_type: helper_fn(cap.get(4)),
                extra: helper_fn(cap.get(5)),
                songs: None,
            })
        },
        None => {
            debug!("No file");
            None
        },
    }

}

pub fn parse_albums(albums: &Vec<String>) -> Option<Vec<Album>> {
    if albums.len() > 0usize {
        let mut vec_albums: Vec<Album> = Vec::new();

        for entry in albums {
            match Path::new(entry).file_name() {
                Some(file) => {
                    match parse_album(file.to_str().unwrap()) {
                        Some(album) => {
                            vec_albums.push(album);
                        },
                        None => (),
                    };
                },
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

    debug!(">>>> Linea recibida {}", song_str);

    let cap = RE.captures(&song_str).unwrap();

    debug!("--->>> Cap: {:?}", cap);

    //Song::new(cap.get(1).map_or("", |m| m.as_str()), cap.get(2).as_str());

    let helper_fn = |x: Option<Match>| x.map_or(None, |m| Some(m.as_str().to_owned()));

    Song {
        number: helper_fn(cap.get(1)),
        name: helper_fn(cap.get(2)),
    }
}

#[cfg(test)]
mod tests {
    use crate::Album;
    use crate::parsers::parse_album;

    #[test]
    fn parse_album_ok() {
        let album = Album::new(Some(String::from("Arctic Monkeys")),
                               Some(String::from("AM")),
                               Some(String::from("2013")),
                               Some(String::from("DD")),
                               Some(String::from(" {16-44, Deezer}")));
        let parsed_album = parse_album("Arctic Monkeys - AM (2013) [DD] {16-44, Deezer}");
        assert_eq!(album, parsed_album);
    }

}
