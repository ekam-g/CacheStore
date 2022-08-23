use std::io;
use std::fs::File;
use std::io::{BufRead, BufReader, Empty, empty, Write};


pub struct Modify();

impl Modify {
    pub fn write(&self, data: String, path: &str) -> Result<Empty, io::Error> {
        let output = File::create(path);
        return match output {
            Ok(mut file) => {
                let error = write!(file, "{}", data);
                match error {
                    Ok(..) => {
                        Ok(empty())
                    }
                    Err(e) => {
                        Err(e)
                    }
                }
            }
            Err(e) => {
                Err(e)
            }
        };
    }

    pub fn read(&self, path: &str) -> Result<Vec<String>, io::Error> {
        let file = File::open(path.to_string());
        let mut v: Vec<String> = Vec::new();
        match file {
            Ok(success) => {
                let reader = BufReader::new(success);
                for line in reader.lines() {
                    match line {
                        Ok(l) => {
                            v.push(l);
                        }
                        Err(e) => {
                            v.push(e.to_string());
                            return Err(e);
                        }
                    }
                }
            }
            Err(error) => {
                v.push(error.to_string());
                return Err(error);
            }
        }
        return Ok(v);
    }
}

