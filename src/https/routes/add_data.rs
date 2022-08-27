use rocket_contrib::json::Json;
use serde::Serialize;
use rocket::get;
use crate::func::files;
use std::fs;


#[derive(Serialize)]
pub struct Data {
    error: String,
}

#[get("/add/<path>/<data_name>/<data>")]
pub fn add(mut path: String, data_name: String, data: String) -> Json<Data> {
    path = path.replace("`", "/");
    let file_error = files::WriteData {}.normal(&data, &format!("{}/{}.txt", &path, &data_name));
    return match file_error {
        Ok(_) => {
            Json(Data {
                error: "Success".to_string(),
            })
        }
        Err(_) => {
            let directory_error = fs::create_dir(&path);
            match directory_error {
                Ok(_) => {
                    let file_error = files::WriteData {}.normal(&data, &format!("{}/{}.txt", path, data_name));
                    match file_error {
                        Ok(_) => {
                            Json(Data {
                                error: "Success".to_string(),
                            })
                        }
                        Err(error) => {
                            Json(Data {
                                error: error.to_string(),
                            })
                        }
                    }
                }
                Err(error) => {
                    Json(Data {
                        error: error.to_string(),
                    })
                }
            }
        }
    }
}


