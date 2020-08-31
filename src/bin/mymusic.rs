#[macro_use]
extern crate clap;
use clap::{App, Arg, SubCommand};
use structopt::StructOpt;

use std::env;

#[macro_use]
extern crate log;
extern crate env_logger;

use env_logger::Env;
use std::path::Path;

// Define struct for command line processing
#[derive(StructOpt, Debug)]
#[structopt(name = "mymusic", about = "My music parser & browser")]
enum Opt {
    #[structopt(name = "parse")]
    Parse {
        #[structopt(
            value_name = "dir",
            about = "The directory where to parse.",
            help = "The directory where to parse."
        )]
        dir: String,
    },
}

fn main() {
    // Init logger
    let env = Env::default()
        .filter_or("LOG_LEVEL", "debug")
        .write_style_or("LOG_STYLE", "always");

    env_logger::init_from_env(env);
    info!("Starting application");

    // Start to process command line
    let opt = Opt::from_args();

    match Opt::from_args() {
        Opt::Parse { dir } => {
            if Path::new(&dir).exists() {
                info!("Path {} exists", &dir);
            } else {
                error!("Wrong path {} provided", &dir);
            }
        }
    }
}

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
