use std::{thread, time};
use crate::files;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {}

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    #[serde(rename = "userId")]
    user_id: i32,
    id: Option<i32>,
    title: String,
    completed: bool,
}

impl Data {
    pub async fn get(&self) {
        let mut x: i64 = 0;
        loop {
            // let todos: Vec<Todo> = reqwest::Client::new()
            //     .get("https://jsonplaceholder.typicode.com/todos?userId=1")
            //     .send()
            //     .await.expect("")
            //     .json()
            //     .await.expect("");
            // println!("{:#?}", todos);

            x = x + 1;
            let output = files::Modify {}.write(x.to_string(), "src/data_getting_test/cache.txt");
            match output {
                Ok(..) => {
                    thread::sleep(time::Duration::from_millis(1000));
                }
                Err(error) => {
                    println!("{}", error);
                }
            }
            thread::sleep(time::Duration::from_millis(100));
        }
    }
}