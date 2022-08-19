#![feature(proc_macro_hygiene, decl_macro)]

use rocket::*;
use rocket_contrib::json::Json;
use rusqlite::Connection;
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
    {
        let db_connection = Connection::open("data.sqlite").unwrap();

        db_connection
            .execute(
                "create table if not exists todo_list (
                    id integer primary key,
                    item varchar(64) not null
                );",
                rusqlite::NO_PARAMS,
            )
            .unwrap();
    }

    rocket::ignite()
        .mount(
            "/",
            routes![index, fetch_all_todo_items, add_todo_item, remove_todo_item,data_test],
        )
        .launch();
}