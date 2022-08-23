use std::fs::File;
use std::io::{Write};
use std::{thread, time};
use crate::files;

pub struct Data{}


impl Data {
    pub fn get(&self) {
        let mut x = 0;
        loop {
            x += 1;
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