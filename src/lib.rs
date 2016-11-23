#![feature(proc_macro)]
#![feature(custom_attribute)]

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate hyper;

pub mod http;
pub mod errors;
#[macro_use]
pub mod query;
mod database;
pub use database::database;
