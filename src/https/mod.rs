use rocket::*;

pub mod routes;

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
                    routes::cashe::key_val_store_read::online_func,
                    routes::cashe::key_val_read::online_func
                ],
            )
            .launch();
    }
}
