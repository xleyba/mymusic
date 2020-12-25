use crate::data::Album;
use crate::error::{MyMusicError, Result};
use serde::{Deserialize, Serialize};
use std::fs;

pub fn serialize_albums(albums: &Vec<Album>) -> Result<()> {
    //let j = serde_json::to_string(albums).unwrap_or("error serializando".to_string());
    let j = serde_json::to_string(albums)?;

    //fs::write("./temp.json", &j).expect("Unable to write file");
    fs::write("./temp.json", &j)?;

    // Print, write to a file, or send to an HTTP server.
    debug!("{}", j);
    Ok(())
}
