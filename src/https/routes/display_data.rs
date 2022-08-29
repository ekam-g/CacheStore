use rocket_contrib::json::Json;
use serde::Serialize;
use rocket::{get, State};
use crate::{func::files, https::StateData};
use super::functions::path_second;

#[derive(Serialize)]
pub struct DataPlaceHolder {
    data: Vec<String>,
    error: String,
}

#[get("/read/<path>/<api_key>")]
pub fn data_test(mut path: String, api_key: String, api_state: State<StateData>) -> Json<DataPlaceHolder> {
    if api_key != api_state.api_key{
        return Json(DataPlaceHolder{
            data : vec!["no data".to_string()],
            error : "Not authorized".to_string()
        })
    }
    path = path_second(path);
    path = path + ".txt";
    let result = files::ReadData {}.read(path);
    return match result {
        Ok(request) => {
            Json(DataPlaceHolder {
                data: request,
                error: "Success".to_string(),
            })
        }
        Err(error) => {
            println!("{}", error);
            Json(DataPlaceHolder {
                data: vec!["no data".to_string()],
                error: error.to_string(),
            })
        }
    };
}


