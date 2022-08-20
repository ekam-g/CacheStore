#![feature(proc_macro_hygiene, decl_macro)]
//this is needed don't delete
use rocket::*;

mod routes;

fn main() {
    ignite()
        .mount(
            "/",
            routes![routes::index,routes::data_test],
        )
        .launch();
}
// rustup override set nightly
// do this so night can be used in the code
// this code will only work with nightly