use std::fmt::Display;

use super::functions::path_second;
use rocket::{get, State};
use rocket_contrib::json::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct DataPlaceHolder {
    pub data: Vec<String>,
    pub error: String,
}

pub struct DisplayFunc {}

impl DisplayFunc {
    pub fn core<T: Display>(&self, path: T, null_key: String) -> DataPlaceHolder {
        let result = txt_writer::ReadData {}.read(format!("{}.txt", path));
        match result {
            Ok(request) => {
                if request[0] == null_key {
                    DataPlaceHolder {
                        data: vec!["null".to_owned()],
                        error: "data is null".to_owned(),
                    }
                } else {
                    DataPlaceHolder {
                        data: request,
                        error: "Success".to_owned(),
                    }
                }
            }
            Err(error) => {
                println!("{}", error);
                DataPlaceHolder {
                    data: vec!["no data".to_owned()],
                    error: error.to_string(),
                }
            }
        }
    }
}

#[get("/read/<path>/<api_key>")]
pub fn read(
    mut path: String,
    api_key: String,
    api_state: State<crate::StateData>,
) -> Json<DataPlaceHolder> {
    if api_key != api_state.api_key {
        return Json(DataPlaceHolder {
            data: vec!["no data".to_owned()],
            error: "Not authorized".to_owned(),
        });
    }
    path = path_second(path, api_state.data_storage_location.clone());
    Json(DisplayFunc {}.core(path, api_state.null.clone()))
}
