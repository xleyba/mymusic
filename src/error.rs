extern crate failure;
use failure::Fail;
use std::io;
use regex::Error;

#[derive(Fail, Debug)]
pub enum MyMusicError {
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),
    /// Serialization or deserialization error
    #[fail(display = "{}", _0)]
    Serde(#[cause] serde_json::Error),
    #[fail(display = "{}", _0)]
    Regex(#[cause] regex::Error),
    #[fail(display = "An unknown error has occurred.")]
    UnknownError,
    #[fail(display = "No files to process")]
    NoFilesError,
}

impl From<io::Error> for MyMusicError {
    fn from(err: io::Error) -> MyMusicError {
        MyMusicError::Io(err)
    }
}

impl From<serde_json::Error> for MyMusicError {
    fn from(err: serde_json::Error) -> MyMusicError {
        MyMusicError::Serde(err)
    }
}

/// Result type for MyMusic
pub type Result<T> = std::result::Result<T, MyMusicError>;
