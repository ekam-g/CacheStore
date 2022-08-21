pub mod routes;

use rocket::*;


pub struct Web {}

pub trait Start {
    fn start(&self);
}


impl Start for Web {
    fn start(&self) {
        ignite()
            .mount(
                "/",
                routes![routes::index,routes::data_test],
            )
            .launch();
    }
}