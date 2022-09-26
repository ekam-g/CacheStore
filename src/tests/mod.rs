#![cfg(test)]


use crate::func::http_request;

mod local_test;
mod cashe_tests;
mod https_test;

pub static BASE_URL: &str = "http://localhost:5000";
// http://0.0.0.0:5000

pub async fn test_url(url: String) {
    let test_data = http_request::Request::read(url).await;
    match test_data {
        Err(e) => {
            panic!("Make Sure Main Code is running\n\n {}", e)
        }
        Ok(data) => {
            if data.error != "Success" {
                panic!("{}", data.error);
            }
        }
    }
}
