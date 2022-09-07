pub mod routes;

use rocket::*;

pub struct Web {}

impl Web {
    pub fn start(&self, init_val: crate::StateData) {
        ignite()
            .manage(init_val)
            .mount(
                "/",
                routes![
                    routes::index,
                    routes::display_data::read,
                    routes::add_data::add,
                    routes::delete::delete,
                    routes::null_write::null_write,
                ],
            )
            .launch();
    }
}
