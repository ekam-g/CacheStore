use rocket::*;
use rocket_contrib::json::Json;
use serde::Serialize;
use crate::files;


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
    let result = files::Modify{}.read(  "src/data_getting_test/cache.txt");
    match result {
        Ok(request) =>{
            v = request;
        }
        Err(error) => {
            error_found = true;
            println!("{}", error);
        }
    }
    Json(DataPlaceHolder { data: v, error: error_found })
}
