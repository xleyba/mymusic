use walkdir::{DirEntry, WalkDir};

fn is_valid(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.contains("[FLAC]") || s.contains("[DD]") || s.contains("[VINYL]"))
        .unwrap_or(false)
}

pub fn get_dirs<'a>(dir_ini: &'a str) -> Vec<String> {
    let mut dirs = Vec::new();

    for entry in WalkDir::new(&dir_ini)
        .into_iter()
        .filter_entry(|e| e.file_type().is_dir()) // && is_valid(e)
        .filter_map(|e| e.ok())
    {
        //println!("Dir: {}, es valid: {}", entry.path().display(), is_valid(&entry));
        if is_valid(&entry) {
            match entry.path().to_str() {
                Some(dir) => dirs.push(String::from(dir)),
                None => (),
            }
        }
    }

    dirs
}

pub fn get_files<'a>(dir_ini: &'a str) -> Vec<String> {
    Vec::new()
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
