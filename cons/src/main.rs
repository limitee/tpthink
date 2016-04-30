#[macro_use]
extern crate cons;
use cons::*;

#[macro_use]
extern crate easy_util;
extern crate rustc_serialize;
use rustc_serialize::json::Json;
use rustc_serialize::json::ToJson;
use std::str::FromStr;



fn main() {
    let err = api_err!("success");
    println!("{}", err);
}
