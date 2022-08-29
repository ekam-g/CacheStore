pub mod display_data;
pub mod add_data;
pub mod delete;

use rocket::*;


#[get("/")]
pub fn index() -> &'static str {
    "Welcome to RustStore!\n\
    The current routes are \n\
    => GET / (index) \n\
    => GET /read/<path> (read data)\n\
    => GET /add/<path>/<data_name>/<data> (add data)\n\
    => GET /delete/<path> (delete data)\n\
    "
}



