

#[derive(Debug)]
// Defined a lifetime for it as it use string references
pub struct Album<'a> {
    artist: Option<&'a str>,
    name: Option<&'a str>,
    year: Option<&'a str>,
    media_type: Option<&'a str>,
    extra: Option<&'a str>,
    songs: Option<Songs<'a>>,
}

impl<'a> Album<'a> {
    // A generic "new" function if you have multiple ways to get
    // initialize Album this may help
    fn new<V>(
        artist: Option<V>,
        name: Option<V>,
        year: Option<V>,
        media_type: Option<V>,
        extra: Option<V>,
        songs: Option<V>,
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


#[derive(Debug)]
// Defined a lifetime for it as it use string references
pub struct Song<'a> {
    number: Option<&'a str>,
    name: Option<&'a str>,
}

#[derive(Debug)]
pub struct Songs<'a> {
    songs: Vec<Song<'a>>,
}

#[derive(Debug)]
pub struct Albums<'a> {
    albums: Vec<Album<'a>>,
}