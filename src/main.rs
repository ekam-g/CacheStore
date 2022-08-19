#![feature(proc_macro_hygiene, decl_macro)]
use rocket::*;

mod routes;

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![routes::index,routes::data_test],
        )
        .launch();
}