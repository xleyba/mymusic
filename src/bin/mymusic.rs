use mymusic::crawlers;
use mymusic::data;
use mymusic::parsers;
use mymusic::serializer;

fn main() {
    /*let mydirs = crawlers::get_dirs("/Users/jleyba/Documents/test");
    println!("Dirs: {:?}", mydirs);

    let myalbums = parsers::parse_albums(&mydirs);

    println!("------");
    println!("{:?}", myalbums);

    println!("------");

    serializer::serialize_albums(&myalbums.unwrap());
    */
    generate_music_collection();
}

fn generate_music_collection() {
    if let Some(mydirs) = crawlers::get_dirs("/Users/jleyba/Documents/test") {
        let mut albums: Vec<data::Album> = Vec::new();

        for entry in mydirs {
            albums.push(parsers::parse_album(&entry));
        }
    } else {
        println!("No files to parse!!");
    }
}
