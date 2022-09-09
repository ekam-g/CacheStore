use crate::https::State;
use better_file_maker;
use rocket::get;
use rocket_contrib::json::Json;
use serde::Serialize;
use txt_writer;

use super::functions::path_second;

#[derive(Serialize)]
pub struct Data {
    pub error: String,
}

pub struct AddDataFunc();

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
    fn make_file_struct(&self, data: String, path: String, data_name: String) -> Data {
        let file_error =
            txt_writer::WriteData {}.drop_replace(data, format!("{}/{}.txt", path, data_name));
        return match file_error {
            Err(e) => Data {
                error: e.to_string(),
            },
            Ok(_) => Data {
                error: "Success".to_string(),
            },
        };
    }
    pub fn core(
        &self,
        final_path: String,
        data_name: String,
        data: String,
        null_key: String,
    ) -> Data {
        let file_error =
            txt_writer::ReadData {}.read(format!("{}/{}.txt", &final_path, &data_name));
        return match file_error {
            Ok(read_data) => {
                if read_data[0] != null_key {
                    let error = txt_writer::WriteData {}
                        .drop_add(data, format!("{}/{}.txt", final_path, data_name));
                    match error {
                        Ok(_) => Data {
                            error: "Success".to_string(),
                        },
                        Err(e) => Data {
                            error: e.to_string(),
                        },
                    }
                } else {
                    return AddDataFunc {}.make_file_struct(data, final_path, data_name);
                }
            }
            Err(_) => {
                let writing_error = txt_writer::WriteData {}
                    .replace(&data, format!("{}/{}.txt", &final_path, &data_name));
                match writing_error {
                    Ok(_) => Data {
                        error: "Success".to_string(),
                    },
                    Err(_) => {
                        let file_error = better_file_maker::make_folders(&final_path);
                        match file_error {
                            Ok(_) => AddDataFunc {}.make_file_struct(data, final_path, data_name),

                            Err(e) => Data {
                                error: e.to_string(),
                            },
                        }
                    }
                }
            }
        };
    }
}

#[get("/add/<path>/<data_name>/<data>/<api_key>")]
pub fn add(
    path: String,
    data_name: String,
    data: String,
    api_key: String,
    api_state: State<crate::StateData>,
) -> Json<Data> {
    if api_key != api_state.api_key {
        return Json(Data {
            error: "Not authorized".to_string(),
        });
    }
    let final_path = path_second(path, api_state.data_storage_location.to_string());
    return Json(AddDataFunc {}.core(final_path, data_name, data, api_state.null.to_string()));
}
