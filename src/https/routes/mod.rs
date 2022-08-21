use std::io::{BufReader, BufRead};
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
    data: Vec<String>,
    error: bool,
}

#[get("/data")]
pub fn data_test() -> Json<DataPlaceHolder> {
    let mut error_found = false;
    let mut v = Vec::new();
    let file = File::open("src/data_getting_test/cache.txt");
    match file {
        Ok(success) => {
            let reader = BufReader::new(success);
            for line in reader.lines() {
                match line {
                    Ok(l) => {
                        v.push(l);
                    }
                    Err(e) => {
                        v.push(e.to_string());
                        error_found = true;
                    }
                }
            }
        }
        Err(error) => {
            v.push(error.to_string());
            error_found = true;
        }
    }
    Json(DataPlaceHolder { data: v, error: error_found })
}
