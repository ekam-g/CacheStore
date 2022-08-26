pub mod display_data;
pub mod add_data;

use rocket::*;


#[get("/")]
pub fn index() -> &'static str {
    "Welcome to Basic Rust API\n\
    The current routes are [/data]"
}



