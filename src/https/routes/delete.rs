use rocket::{get, State};
use rocket_contrib::json::Json;
use serde::Serialize;
use std::{fmt::Display, fs};

use super::functions::path_second;

#[derive(Serialize)]
pub struct Data {
    pub error: String,
}
pub struct DeleteFunc {}

impl DeleteFunc {
    pub fn main_func<T: Display>(&self, path: T) -> Data {
        let delete_error = fs::remove_file(format!("{}.txt", path));
        match delete_error {
            Ok(_) => Data {
                error: "Success".to_string(),
            },
            Err(error) => Data {
                error: error.to_string(),
            },
        }
    }
}

#[get("/delete/<path>/<api_key>")]
pub fn delete(path: String, api_key: String, api_state: State<crate::StateData>) -> Json<Data> {
    if api_key != api_state.api_key {
        return Json(Data {
            error: "Not authorized".to_string(),
        });
    }
    let final_path = path_second(path, api_state.data_storage_location.to_string());
    return Json(DeleteFunc {}.main_func(final_path));
}
