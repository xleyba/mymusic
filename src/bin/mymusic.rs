fn main() {}

/*
fn generate_music_collection() {
    if let Some(mydirs) = crawlers::get_dirs("/Users/jleyba/Documents/test").unwrap() {
        let mut albums: Vec<data::Album> = Vec::new();

        for entry in mydirs {
            println!("Entry: {}", &entry);

            let mut album = parsers::parse_album(&entry);

            if let Some(myfiles) = crawlers::get_files(&entry) {
                println!("MyFiles: {:?}", myfiles);
                let mut songs_v: Vec<data::Song> = Vec::new();

                for esong in myfiles {
                    songs_v.push(parsers::parse_song(&esong));
                }

                if songs_v.len() > 0 {
                    let songs_o = data::Songs { songs: songs_v };
                    album.songs = Some(songs_o);
                }

                albums.push(album);
            } else {
                albums.push(album);
            }

            serializer::serialize_albums(&albums);
        }
    } else {
        println!("No files");
    }
}

*/
