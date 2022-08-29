pub mod display_data;
pub mod add_data;
pub mod delete;

use rocket::*;


#[get("/")]
pub fn index() -> &'static str {
    "Welcome to RustStore!\n\
    The current routes are \n\
    => GET / (index) \n\
    => GET /read/<path>/<api_key> (read data)\n\
    => GET /add/<path>/<data_name>/<data>/<api_key> (add data)\n\
    => GET /delete/<path>/<api_key> (delete data)\n\
    "
}



