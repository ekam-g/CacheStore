pub mod routes;

use rocket::*;


pub struct Web {}


impl Web {
    pub(crate) fn start(&self) {
        ignite()
            .mount(
                "/",
                routes![routes::index,routes::data_test],
            )
            .launch();
    }
}