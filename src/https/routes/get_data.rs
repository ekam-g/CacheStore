use rocket_contrib::json::Json;
use serde::Serialize;
use rocket::get;
use crate::func::files;

#[derive(Serialize)]
pub struct DataPlaceHolder {
    data: Vec<String>,
    error: bool,
}

#[get("/data")]
pub fn data_test() -> Json<DataPlaceHolder> {
    let result = files::Modify {}.read("src/data_getting_test/cache.txt");
    return match result {
        Ok(request) => {
            Json(DataPlaceHolder {
                data: request,
                error: false,
            })
        }
        Err(error) => {
            println!("{}", error);
            Json(DataPlaceHolder {
                data: vec![error.to_string()],
                error: true,
            })
        }
    };
}