#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;
extern crate env_logger;

pub mod crawlers;
pub mod data;
pub mod error;
pub mod parsers;
pub mod serializer;

pub use crawlers::get_dirs;
pub use data::*;
pub use error::{MyMusicError, Result};
pub use parsers::parse_albums;
pub use serializer::serialize_albums;
