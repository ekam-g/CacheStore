use std::io::{Write, BufReader, BufRead, Error};
use std::{thread, time};
use std::fs::File;
use rocket::*;
use rocket_contrib::json::Json;
use serde::Serialize;




#[get("/")]
pub fn index() -> &'static str {
    "Welcome to Basic Rust API\n\
    The current routes are [/data]"

}
#[derive(Serialize)]

pub struct DataPlaceHolder {
    data : Vec<String>,
}

#[get("/data")]
pub fn data_test() -> Json<DataPlaceHolder> {
    let path = "src/data_getting_test/cache.txt";
    let input = File::open(path).expect("file not found");
    let buffered = BufReader::new(input);
    let mut v = Vec::new();
    for line in buffered.lines() {
        v.push(line);
    }
    let content: String = "the data page worked".to_string();
    Json(DataPlaceHolder { data : v })
}
