#[macro_use]
extern crate clap;
extern crate env_logger;
#[macro_use]
extern crate log;

extern crate spinners;

use spinners::{Spinner, Spinners};
use std::thread::sleep;
use std::time::Duration;

use std::{env, thread, io};
use std::path::Path;

use clap::{App, Arg, SubCommand};
use env_logger::Env;
use structopt::StructOpt;

use mymusic::{crawlers, data, parsers, serializer};
use std::io::Write;


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
        .filter_or("LOG_LEVEL", "info")
        .write_style_or("LOG_STYLE", "always");

    env_logger::init_from_env(env);
    info!("Starting application");

    // Start to process command line
    let opt = Opt::from_args();

    match Opt::from_args() {
        Opt::Parse { dir } => {
            if Path::new(&dir).exists() {
                debug!("Path {} exists", &dir);
                generate_music_collection(&dir)
            } else {
                error!("Wrong path {} provided", &dir);
            }
        }
    }
}


fn generate_music_collection(dir_ini: &str) {

}

