use crate::https::{State, StateData};
use better_file_maker;
use rocket::get;
use rocket_contrib::json::Json;
use serde::Serialize;
use txt_writer;

use super::functions::path_second;

#[derive(Serialize)]
pub struct Data {
    error: String,
}

struct AddDataFunc();

impl AddDataFunc {
    fn make_file(&self, data: String, path: String, data_name: String) -> Json<Data> {
        let file_error =
            txt_writer::WriteData {}.drop_replace(data, format!("{}/{}.txt", path, data_name));
        return match file_error {
            Err(e) => Json(Data {
                error: e.to_string(),
            }),
            Ok(_) => Json(Data {
                error: "Success".to_string(),
            }),
        };
    }
}

#[get("/add/<path>/<data_name>/<data>/<api_key>")]
pub fn add(
    path: String,
    data_name: String,
    data: String,
    api_key: String,
    api_state: State<StateData>,
) -> Json<Data> {
    if api_key != api_state.api_key {
        return Json(Data {
            error: "Not authorized".to_string(),
        });
    }
    let final_path = path_second(path);
    let file_error = txt_writer::ReadData {}.read(format!("{}/{}.txt", &final_path, &data_name));
    match file_error {
        Ok(read_data) => {
            if read_data[0] != api_state.null {
                let error = txt_writer::WriteData {}
                    .drop_add(data, format!("{}/{}.txt", final_path, data_name));
                match error {
                    Ok(_) => {
                        return Json(Data {
                            error: "Success".to_string(),
                        })
                    }
                    Err(e) => {
                        return Json(Data {
                            error: e.to_string(),
                        })
                    }
                }
            } else {
                return AddDataFunc {}.make_file(data, final_path, data_name);
            }
        }
        Err(_) => {
            let writing_error = txt_writer::WriteData {}
                .replace(&data, format!("{}/{}.txt", &final_path, &data_name));
            match writing_error {
                Ok(_) => {
                    return Json(Data {
                        error: "Success".to_string(),
                    })
                }
                Err(_) => {
                    let _ = better_file_maker::make_folders(&final_path);
                    return AddDataFunc {}.make_file(data, final_path, data_name);
                }
            }
        }
    };
}
