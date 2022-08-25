use std::{thread, time};
use std::io::Error;
use crate::func::files;
use crate::func::http_request;

pub struct Data {}


impl Data {
    pub async fn get(&self) {
        loop {
            let result_data = http_request::Request::read().await;
            match result_data {
                Ok(data) => {
                    let mut error_found = false;
                    let mut amount: i8 = 0;
                    let mut output: Result<(), Error>;
                    for data in &data.data {
                        if amount == 0 {
                            output = files::WriteData {}.replace(data.perm.to_string(), "src/data_getting_test/cache.txt");
                        } else {
                            output = files::WriteData {}.normal(data.perm.to_string(), "src/data_getting_test/cache.txt");
                        }
                        match output {
                            Ok(..) => {}
                            Err(error) => {
                                error_found = true;
                                println!("{}", error);
                            }
                        }
                        amount = amount + 1;
                    }
                    if !error_found {
                        thread::sleep(time::Duration::from_secs(10));
                    }
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            }
            thread::sleep(time::Duration::from_millis(100));
        }
    }
}