use std::{thread, time};
use crate::func::files;
use crate::func::http_request;
pub struct Data {}


impl Data {
    pub async fn get(&self) {
        loop {
            let data = http_request::Request::read().await;
            let output = files::Modify {}.write(data.data[0].perm.to_string(), "src/data_getting_test/cache.txt");
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