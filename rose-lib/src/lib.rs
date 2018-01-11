// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

extern crate byteorder;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;

pub mod io;
pub mod utils;
pub mod errors;

pub mod files;

