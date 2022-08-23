pub mod routes;
use rocket::*;


pub struct Web {}


impl Web {
    pub fn start(&self) {
        ignite()
            .mount(
                "/",
                routes![routes::index,routes::get_data::data_test],
            )
            .launch();
    }
}