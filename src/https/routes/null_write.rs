use rocket::{get, State};
use rocket_contrib::json::Json;
use serde::Serialize;
use std::fmt::Display;
use txt_writer;

use super::functions::path_second;

#[derive(Serialize)]
pub struct Data {
    pub error: String,
}

pub struct NullFunc {}

impl NullFunc {
    pub fn core<T: Display>(&self, null_key: String, path: T) -> Data {
        let delete_error = txt_writer::WriteData {}.replace(&null_key, format!("{}.txt", path));
        match delete_error {
            Ok(_) => Data {
                error: "Success".to_owned(),
            },
            Err(error) => Data {
                error: error.to_string(),
            },
        }
    }
}

#[get("/null_write/<path>/<api_key>")]
pub fn null_write(path: String, api_key: String, api_state: State<crate::StateData>) -> Json<Data> {
    if api_key != api_state.api_key {
        return Json(Data {
            error: "Not authorized".to_string(),
        });
    }
    let final_path = path_second(path, api_state.data_storage_location.clone());
    return Json(NullFunc {}.core(api_state.null.clone(), final_path));
}
