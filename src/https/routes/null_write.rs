use rocket::{get, State};
use rocket_contrib::json::Json;
use serde::Serialize;
use txt_writer;

use crate::https::StateData;

use super::functions::path_second;

#[derive(Serialize)]
pub struct Data {
    error: String,
}

struct NullFunc {}

impl NullFunc {
    fn core(&self, null_key: String, path: String) -> Json<Data> {
        let delete_error = txt_writer::WriteData {}.replace(&null_key.to_string(), path);
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

#[get("/null_write/<path>/<api_key>")]
pub fn null_write(path: String, api_key: String, api_state: State<StateData>) -> Json<Data> {
    if api_key != api_state.api_key {
        return Json(Data {
            error: "Not authorized".to_string(),
        });
    }
    let final_path = path_second(path);
    return NullFunc {}.core(api_state.null.to_string(), final_path);
}
