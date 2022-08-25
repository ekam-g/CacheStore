use std::fs::{OpenOptions};
use std::io::{Empty, empty, Error, Write};
use files::WriteData;
use crate::func::files;

impl WriteData {
    pub fn normal(&self, data: String, path: &str) -> Result<Empty, Error> {
        let  output = OpenOptions::new()
            .write(true)
            .append(true)
            .open(path);
        return match output {
            Ok(mut file) => {
                let error = write!(file, "{}\n", data);
                match error {
                    Ok(..) => {
                        file.flush()?;
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