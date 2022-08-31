use crate::func::files;
use files::WriteData;
use std::fs::{File, OpenOptions};
use std::io::{Error, Write};

impl WriteData {
    pub fn drop_normal(&self, data: String, path: String) -> Result<(), Error> {
        WriteData {}.normal(&data, path)
    }
    pub fn drop_replace(&self, data: String, path: String) -> Result<(), Error> {
        WriteData {}.replace(&data, path)
    }
    pub fn normal(&self, data: &String, path: String) -> Result<(), Error> {
        let output = OpenOptions::new().write(true).append(true).open(path);
        return WriteData {}.process(output, &data);
    }
    pub fn replace(&self, data: &String, path: String) -> Result<(), Error> {
        let output = File::create(path);
        return WriteData {}.process(output, &data);
    }
    fn process(&self, output: Result<File, Error>, data: &String) -> Result<(), Error> {
        return match output {
            Ok(mut file) => {
                let error = write!(file, "{}\n", data);
                file.flush()?;
                match error {
                    Ok(..) => Ok(()),
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        };
    }
}
