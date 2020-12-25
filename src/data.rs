use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
// Using references to avoid allocating for every string.Album.Album
// Defined a lifetime for it as it use string references
pub struct Album {
    pub artist: Option<String>,
    pub name: Option<String>,
    pub year: Option<String>,
    pub media_type: Option<String>,
    pub extra: Option<String>,
    pub songs: Option<Songs>,
}

impl<'a> Album {
    // A generic "new" function if you have multiple ways to get
    // initialize Album this may help
    pub fn new(
        artist: Option<String>,
        name: Option<String>,
        year: Option<String>,
        media_type: Option<String>,
        extra: Option<String>,
    ) -> Self {
        // helper function to map the inputs into the fields
        let helper_fn = |x: Option<String>| x.map_or(None, |m| Some(m.to_owned()));

        Album {
            artist: helper_fn(artist),
            name: helper_fn(name),
            year: helper_fn(year),
            media_type: helper_fn(media_type),
            extra: helper_fn(extra),
            songs: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize,PartialEq, Eq)]
// Defined a lifetime for it as it use string references
pub struct Song {
    pub number: Option<String>,
    pub name: Option<String>,
}

impl Song {
    // A generic "new" function if you have multiple ways to get
    // initialize Album this may help
    pub fn new(number: Option<String>, name: Option<String>) -> Self {
        // helper function to map the inputs into the fields
        let helper_fn = |x: Option<String>| x.map_or(None, |m| Some(m.to_owned()));

        Song {
            number: helper_fn(number),
            name: helper_fn(name),
        }
    }
}

#[derive(Debug, Serialize, Deserialize,PartialEq, Eq)]
pub struct Songs {
    pub songs: Option<Vec<Song>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Albums {
    albums: Option<Vec<Album<>>>,
}
