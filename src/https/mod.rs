pub mod routes;

use rocket::*;


pub struct Web {}

pub struct StateData {
    api_key : String
}

impl Web {
    pub fn start(&self) {
        let state  = StateData{
            api_key : "your_api_key".to_string()
        };
        ignite()
        .manage(state)
            .mount(
                "/",
                routes![routes::index,routes::display_data::data_test, routes::add_data::add, routes::delete::delete],
            )
            .launch();
    }
}