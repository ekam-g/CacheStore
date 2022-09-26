use std::collections::HashMap;
use std::fmt::Display;

use rocket::{get, State};
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::https::routes::key_val_store_read::RAM_STORE;

#[derive(Serialize)]
pub struct Data {
    pub data: String,
    pub error: String,
}

pub struct KeyFunc {}

impl KeyFunc {
    pub fn main_func<T: Display, >(&self, key: T) -> Data {
        let read_error = RAM_STORE.lock();
        match read_error {
            Err(error_data) =>
                Data {
                    data: "error".to_owned(),
                    error: error_data.to_string(),
                },

            Ok(cash_data) => {
                let hash_error = HashMap::get(&cash_data, &key.to_string());
                match hash_error {
                    None => Data {
                        data: "error".to_owned(),
                        error: "no key is stored".to_owned(),
                    },
                    Some(final_data) => Data {
                        data: final_data.clone(),
                        error: "Success".to_owned(),
                    }
                }
            }
        }
    }
}

#[get("/read_key/<key>/<api_key>")]
pub fn online_func(key: String, api_key: String, api_state: State<crate::StateData>) -> Json<Data> {
    if api_key != api_state.api_key {
        return Json(Data {
            data: "error".to_owned(),
            error: "Not authorized".to_owned(),
        });
    }
    Json(KeyFunc {}.main_func(key))
}
