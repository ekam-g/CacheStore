pub mod routes;

use rocket::*;


pub struct Web {}


impl Start {
    fn start(&self) {
        ignite()
            .mount(
                "/",
                routes![routes::index,routes::data_test],
            )
            .launch();
    }
}