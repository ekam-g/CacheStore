use std::fs;
use rocket_contrib::json::Json;
use serde::Serialize;
use rocket::get;
use crate::func::files;

#[derive(Serialize)]
pub struct Data {
    error: String,
}

#[get("/delete/<path>")]
pub fn delete(mut path: String) -> Json<Data> {
    path = path.replace("`", "/");
    path = "database/".to_string() + &*path + ".txt";
    let delete_error = fs::remove_file(path);
    match delete_error {
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


