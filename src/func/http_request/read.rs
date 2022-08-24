use crate::func::http_request::Request;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
   #[serde(rename = "userId")]
   user_id: i32,
   id: Option<i32>,
   title: String,
   completed: bool,
}

impl Request{
   pub async fn read() -> Vec<Todo>{
      let todos: Vec<Todo> = reqwest::Client::new()
          .get("https://jsonplaceholder.typicode.com/todos?userId=1")
          .send()
          .await.expect("")
          .json()
          .await.expect("");
      println!("{:#?}", todos[1].title);
      return todos;
   }
}