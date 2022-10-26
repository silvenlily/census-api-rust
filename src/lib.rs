// project linter rules:
#![allow(clippy::needless_return)] // returning if your missing a semicolon is a very bad idea
#![allow(clippy::needless_late_init)] // let statment with an initializer is significantly less readable
                                      // modules
pub mod events;
pub mod rest;
pub mod shared;
pub mod utils;
