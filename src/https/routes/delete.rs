use std::fs;
use rocket_contrib::json::Json;
use serde::Serialize;
use rocket::{get, State};

use crate::https::StateData;

use super::functions::path_second;


#[derive(Serialize)]
pub struct Data {
    error: String,
}

#[get("/delete/<path>/<api_key>")]
pub fn delete(mut path: String, api_key: String, api_state : State<StateData>) -> Json<Data> {
    if api_key != api_state.api_key{
        return Json(Data{
            error : "Not authorized".to_string()
        })
    }
    path = path_second(path);
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


