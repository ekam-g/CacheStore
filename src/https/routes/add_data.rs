use rocket_contrib::json::Json;
use serde::Serialize;
use rocket::get;
use crate::func::files;
use std::fs;


#[derive(Serialize)]
pub struct Data {
    error: String,
}

struct AddDataFunc();

impl AddDataFunc {
    fn make_file(&self, data: String, path: String, data_name: String) -> Json<Data> {
        let file_error = files::WriteData {}.drop_replace(data, format!("{}/{}.txt", path, data_name));
        return match file_error {
            Err(e) => {
                Json(Data {
                    error: e.to_string(),
                })
            }
            Ok(_) => {
                Json(Data {
                    error: "Success".to_string(),
                })
            }
        };
    }
}

#[get("/add/<path>/<data_name>/<data>")]
pub fn add(mut path: String, data_name: String, data: String) -> Json<Data> {
    path = path.replace("`", "/");
    path = "database/".to_string() + &*path;
    let file_error = files::WriteData {}.replace(&data, format!("{}/{}.txt", &path, &data_name));
    return match file_error {
        Ok(_) => {
            Json(Data {
                error: "Success".to_string(),
            })
        }
        Err(_) => {
            let directory_error = fs::create_dir(&path);
            return match directory_error {
                Ok(_) => {
                    AddDataFunc {}.make_file(data, path, data_name)
                }
                Err(_) => {
                    let full: String = path.replace("database/", "");
                    let where_file = full.split("/");
                    let mut location: String = "database/".to_string();
                    for i in where_file {
                        location = location + i + "/";
                        let directory_error = fs::create_dir(&location);
                        match directory_error {
                            Ok(_) => {}
                            Err(error) => {
                                return Json(Data {
                                    error: error.to_string(),
                                });
                            }
                        };
                    }
                    AddDataFunc {}.make_file(data, path, data_name)
                }
            };
        }
    };
}

