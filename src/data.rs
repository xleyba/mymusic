use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
// Using references to avoid allocating for every string.Album.Album
// Defined a lifetime for it as it use string references
pub struct Album<'a> {
    artist: Option<&'a str>,
    name: Option<&'a str>,
    year: Option<&'a str>,
    media_type: Option<&'a str>,
    extra: Option<&'a str>,
    songs: Option<Songs>,
}

impl<'a> Album<'a> {
    // A generic "new" function if you have multiple ways to get
    // initialize Album this may help
    pub fn new<V>(
        artist: Option<V>,
        name: Option<V>,
        year: Option<V>,
        media_type: Option<V>,
        extra: Option<V>,
    ) -> Self
    where
        V: Into<&'a str>,
    {
        // helper function to map the inputs into the fields
        let helper_fn = |x: Option<V>| x.map_or(None, |m| Some(m.into()));

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

#[derive(Debug, Serialize, Deserialize)]
// Defined a lifetime for it as it use string references
pub struct Song {
    number: Option<String>,
    name: Option<String>,
}

impl Song {
    // A generic "new" function if you have multiple ways to get
    // initialize Album this may help
    pub fn new(
        number: Option<String>,
        name: Option<String>,
    ) -> Self
    {
        // helper function to map the inputs into the fields
        let helper_fn = |x: Option<String>| x.map_or(None, |m| Some(m));

        Song {
            number: helper_fn(number),
            name: helper_fn(name),
        }
    }
}




#[derive(Debug, Serialize, Deserialize)]
pub struct Songs {
    songs: Vec<Song>,
}

/* #[derive(Debug, Serialize, Deserialize)]
pub struct Albums<'a> {
    albums: Vec<Album<'a>>,
}
*/
