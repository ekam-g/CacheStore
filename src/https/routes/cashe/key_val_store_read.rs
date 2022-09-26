use std::collections::HashMap;
use std::fmt::Display;
use std::sync::Mutex;

use once_cell::sync::Lazy;
use rocket::{get, State};
use rocket_contrib::json::Json;
use serde::Serialize;

pub static RAM_STORE: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});


#[derive(Serialize)]
pub struct Data {
    pub error: String,
}

pub struct KeyFunc {}

impl KeyFunc {
    pub fn main_func<T: Display, E: Display>(&self, key: T, data: E) -> Data {
        let edit_error = RAM_STORE.lock();
        match edit_error {
            Err(error_data) =>
                Data {
                    error: error_data.to_string(),
                },

            Ok(mut cash_data) => {
                cash_data.insert(key.to_string(), data.to_string());
                Data {
                    error: "Success".to_owned(),
                }
            }
        }
    }
}

#[get("/add_key/<key>/<data>/<api_key>")]
pub fn online_func(key: String, data: String, api_key: String, api_state: State<crate::StateData>) -> Json<Data> {
    if api_key != api_state.api_key {
        return Json(Data {
            error: "Not authorized".to_owned(),
        });
    }
    unsafe {
        Json(KeyFunc {}.main_func(key, data))
    }
}
