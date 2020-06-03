use crate::error::Result;
use walkdir::{DirEntry, WalkDir};

fn is_valid_dir(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.contains("[FLAC]") || s.contains("[DD]") || s.contains("[VINYL]"))
        .unwrap_or(false)
}

fn is_valid_file(entry: &DirEntry) -> bool {
    entry
        .path()
        .extension()
        .unwrap()
        .to_str()
        .map(|s| s.contains("flac"))
        .unwrap_or(false)
}

pub fn get_dirs<'a>(dir_ini: &'a str) -> Result<Option<Vec<String>>> {
    let mut dirs = Vec::new();

    for entry in WalkDir::new(&dir_ini)
        .into_iter()
        .filter_entry(|e| e.file_type().is_dir()) // && is_valid(e)
        .filter_map(|e| e.ok())
    {
        println!(
            "Dir: {}, es valid: {}",
            entry.path().display(),
            is_valid_dir(&entry)
        );
        if is_valid_dir(&entry) {
            match entry.path().to_str() {
                Some(dir) => dirs.push(String::from(dir)),
                None => (),
            }
        }
    }

    if dirs.len() > 0 {
        return Ok(Some(dirs));
    }

    Ok(None)
}

pub fn get_files<'a>(dir_ini: &'a str) -> Option<Vec<String>> {
    let mut songs = Vec::new();

    for entry in WalkDir::new(&dir_ini).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file()
            && entry
                .path()
                .extension()
                .unwrap()
                .to_str()
                .unwrap()
                .contains("flac")
        {
            println!("Linea file: {}", entry.path().to_str().unwrap());
            songs.push(String::from(
                entry.path().file_stem().unwrap().to_str().unwrap(),
            ));
        }
    }

    if songs.len() > 0 {
        return Some(songs);
    }

    None
}

// Unit test
#[cfg(test)]
mod tests {

    use super::*;

    /* #[test]
    fn ok_values() {
        let album = parse_album("myArtist - myAlbum (2020) [FLAC] {extra info}").unwrap();
        assert_eq!("myArtist", album.artist.unwrap());
    }*/
}
