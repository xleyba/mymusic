use mymusic::crawlers;
use mymusic::parsers;
use mymusic::serializer;

fn main() {
    let mydirs = crawlers::get_dirs("/Users/jleyba/Documents/test");
    println!("Dirs: {:?}", mydirs);

    let myalbums = parsers::parse_albums(&mydirs);

    println!("------");
    println!("{:?}", myalbums);

    println!("------");

    serializer::serialize_albums(&myalbums.unwrap());
}
