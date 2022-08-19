#![feature(proc_macro_hygiene, decl_macro)]

use rocket::*;
use rocket_contrib::json::Json;
use serde::Serialize;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
#[derive(Serialize)]
struct DataPlaceHolder {
    data : String,
}

#[get("/data")]
fn data_test() -> Json<DataPlaceHolder> {
    Json(DataPlaceHolder { data : "yes".parse().unwrap() })
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![index,data_test],
        )
        .launch();
}