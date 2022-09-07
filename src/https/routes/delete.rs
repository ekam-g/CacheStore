use rocket::{get, State};
use rocket_contrib::json::Json;
use serde::Serialize;
use std::fs;

use crate::https::StateData;

use super::functions::path_second;

#[derive(Serialize)]
pub struct Data {
    error: String,
}
struct DeleteFunc {}

impl DeleteFunc {
    pub fn main_func(&self, path: String) -> Json<Data> {
        let delete_error = fs::remove_file(path);
        match delete_error {
            Ok(_) => Json(Data {
                error: "Success".to_string(),
            }),
            Err(error) => Json(Data {
                error: error.to_string(),
            }),
        }
    }
}

#[get("/delete/<path>/<api_key>")]
pub fn delete(path: String, api_key: String, api_state: State<StateData>) -> Json<Data> {
    if api_key != api_state.api_key {
        return Json(Data {
            error: "Not authorized".to_string(),
        });
    }
    let final_path = path_second(path);
    return DeleteFunc {}.main_func(final_path);
}
