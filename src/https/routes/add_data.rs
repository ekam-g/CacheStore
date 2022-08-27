use rocket_contrib::json::Json;
use serde::Serialize;
use rocket::get;
use crate::func::files;
use std::fs;
use std::io::{Error, ErrorKind};
use std::ops::Add;


#[derive(Serialize)]
pub struct Data {
    error: String,
}

#[get("/add/<path>/<data_name>/<data>")]
pub fn add(mut path: String, data_name: String, data: String) -> Json<Data> {
    path = path.replace("`", "/");
    path = "database/".to_string() + &*path;
    let file_error = files::WriteData {}.replace(&data, &format!("{}/{}.txt", &path, &data_name));
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
                    let file_error = files::WriteData {}.replace(&data, &format!("{}/{}.txt", path, data_name));
                    match file_error {
                        Ok(_) => {
                            Json(Data {
                                error: "Success".to_string(),
                            })
                        }
                        Err(error) => {
                            match error.kind() {
                                ErrorKind::NotADirectory => {
                                    let where_file = path.split("`");
                                    for i in where_file {
                                        let directory_error = fs::create_dir(i);
                                        match directory_error {
                                            Ok(_) => {}
                                            Err(what) => {
                                                match what.kind() {
                                                    ErrorKind::AlreadyExists => {}
                                                    other_error => {
                                                        return Json(Data {
                                                            error: "Error".to_string(),
                                                        });
                                                    }
                                                }
                                            }
                                        };
                                    }
                                }
                                other_error => {
                                    Json(Data {
                                        error: "Error".to_string(),
                                    })
                                }
                            }
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
    };
}


