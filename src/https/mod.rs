pub mod routes;

use rocket::*;


pub struct Web {}

pub struct State {
    api_key : String
}

impl Web {
    pub fn start(&self) {
        let state: State  = State{
            api_key : "gdfjshadgfa2432h4g51u4o5".to_string()
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