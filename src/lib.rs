#[macro_use]
extern crate lazy_static;

pub mod data;
pub mod crawlers;
pub mod parsers;

pub use crawlers::get_dirs;
pub use parsers::parse_albums;
pub use data::*;