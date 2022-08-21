use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};
use std::{thread, time};

pub fn data_get(){
    let mut x = 0;
    let panicMessage : &str = "TODO: panic message";
    ///todo rework panics
    loop{
        x += 1;
        let path = "src/data_getting_test/cache.txt";
        let mut output = File::create(path).expect(panicMessage);
        write!(output, "{}", x).expect(panicMessage);
        thread::sleep(time::Duration::from_millis(1000));
    }
}

