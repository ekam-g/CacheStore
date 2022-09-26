use std::collections::HashMap;
use std::fmt::Display;

use rocket::{get, State};
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::https::routes::cashe::key_val_store_read::RAM_STORE;

#[derive(Serialize)]
pub struct Data {
    pub error: String,
}

pub struct KeyFuncDel {}

impl KeyFuncDel {
    pub fn main_func<T: Display, >(&self, key: T) -> Data {
        let read_error = RAM_STORE.lock();
        match read_error {
            Err(error_data) =>
                Data {
                    error: error_data.to_string(),
                },

            Ok(mut cash_data) => {
                let hash_error = HashMap::remove(&mut cash_data, &*key.to_string());
                match hash_error {
                    None => Data {
                        error: "no key is stored".to_owned(),
                    },
                    Some(_) => Data {
                        error: "Success".to_owned(),
                    }
                }
            }
        }
    }
}

#[get("/delete_key/<key>/<api_key>")]
pub fn online_func(key: String, api_key: String, api_state: State<crate::StateData>) -> Json<Data> {
    if api_key != api_state.api_key {
        return Json(Data {
            error: "Not authorized".to_owned(),
        });
    }
    Json(KeyFuncDel {}.main_func(key))
}
