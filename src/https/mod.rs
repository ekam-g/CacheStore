pub mod routes;

use rocket::*;




pub fn start(){
    ignite()
        .mount(
            "/",
            routes![routes::index,routes::data_test],
        )
        .launch();
}