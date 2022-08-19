
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
    data : String,
}

#[get("/data")]
pub fn data_test() -> Json<DataPlaceHolder> {
    Json(DataPlaceHolder { data : "yes".parse().unwrap() })
}
