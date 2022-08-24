use std::{thread, time};
use serde::{Serialize, Deserialize};
use crate::func::files;
use crate::func::http_request;
pub struct Data {}


impl Data {
    pub async fn get(&self) {
        let mut x: i64 = 0;
        loop {

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