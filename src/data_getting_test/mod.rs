use std::fs::File;
use std::io::{Write};
use std::{thread, time};

pub fn data_get(){
    let mut x = 0;
    let panic_message: &str = "TODO: panic message";
    //todo panic message
    loop{
        x += 1;
        let path = "src/data_getting_test/cache.txt";
        let mut output = File::create(path).expect(panic_message);
        write!(output, "{}", x).expect(panic_message);
        thread::sleep(time::Duration::from_millis(1000));
    }
}

