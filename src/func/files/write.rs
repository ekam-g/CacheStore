use std::fs::File;
use std::io::{Empty, empty, Error, Write};
use files::Modify;
use crate::func::files;

impl Modify {
    pub fn write(&self, data: String, path: &str) -> Result<Empty, Error> {
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