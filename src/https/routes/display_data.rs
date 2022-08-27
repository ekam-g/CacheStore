use rocket_contrib::json::Json;
use serde::Serialize;
use rocket::get;
use crate::func::files;

#[derive(Serialize)]
pub struct DataPlaceHolder {
    data: Vec<String>,
    error: String,
}

#[get("/read/<path>")]
pub fn data_test(mut path: String) -> Json<DataPlaceHolder> {
    path = path.replace("`", "/");
    path = "database/".to_string() + &*path + ".txt";
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


