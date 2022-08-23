use crate::files::Modify;
use std::io;
use std::fs::File;
use std::io::{Empty, empty, Write};

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
}