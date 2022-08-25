use std::error::Error;
use crate::func::files::Files;
use std::fs;

impl Files{
    pub fn create_folder(&self, path : String ) -> Result<(), dyn Error>{
        fs::create_dir(path)
    }
}