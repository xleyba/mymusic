use crate::data::Album;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs;

pub fn serialize_albums(albums: &Vec<Album>) {
    let j = serde_json::to_string(albums).unwrap_or("error serializando".to_string());

    fs::write("./temp.json", &j).expect("Unable to write file");

    // Print, write to a file, or send to an HTTP server.
    println!("{}", j);
}
