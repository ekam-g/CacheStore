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
    pub fn core(&self, path: String, null_key: String) -> DataPlaceHolder {
        let final_path = path + ".txt";
        let result = txt_writer::ReadData {}.read(final_path);
        return match result {
            Ok(request) => {
                if request[0] == null_key {
                    return DataPlaceHolder {
                        data: vec!["null".to_string()],
                        error: "data is null".to_string(),
                    };
                } else {
                    return DataPlaceHolder {
                        data: request,
                        error: "Success".to_string(),
                    };
                }
            }
            Err(error) => {
                println!("{}", error);
                DataPlaceHolder {
                    data: vec!["no data".to_string()],
                    error: error.to_string(),
                }
            }
        };
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
            data: vec!["no data".to_string()],
            error: "Not authorized".to_string(),
        });
    }
    path = path_second(path, api_state.data_storage_location.to_string());
    return Json(DisplayFunc {}.core(path, api_state.null.to_string()));
}
