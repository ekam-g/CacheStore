pub mod routes;

use rocket::*;
use crate::database_func;

pub struct Web {}

pub struct StateData {
    api_key : String
}

impl Web {
    pub fn start(&self) {
        let state  = StateData{
            api_key : "your_api_key".to_string()
            //TODO when using this please write an api key
        };
        tokio::spawn(async {
            database_func::Func {}.example().await;
            // use this if you want to spawn aync funtions to modify values in state
         });
        ignite()
        .manage(state)
            .mount(
                "/",
                routes![routes::index,routes::display_data::data_test, routes::add_data::add, routes::delete::delete],
            )
            .launch();
    }
}