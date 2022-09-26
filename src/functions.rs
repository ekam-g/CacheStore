use std::fmt::Display;

use better_file_maker;

use crate::StateData;

impl StateData {
    fn error_or_not(&self, error: String) -> Result<(), String> {
        if error == "Success" {
            println!("Response succeeded!!!!");
            Ok(())
        } else {
            println!("FAILED!!!!");
            Err(error)
        }
    }
    fn path_format(&self, path: String) -> String {
        format!("{}{}", self.data_storage_location, path)
    }

    pub fn start_database_online(self) {
        let error = better_file_maker::make_folders(&self.data_storage_location);
        match error {
            Err(e) => {
                println!(
                    "When starting the database and creating a folder {} error was received",
                    e
                );
                crate::https::Web {}.start(self);
            }
            Ok(_) => {
                crate::https::Web {}.start(self);
            }
        }
    }
    pub fn write_data<T: Display, T2: Display, T3: Display>(
        &self,
        data: T,
        path: T2,
        data_name: T3,
    ) -> Result<(), String> {
        let final_path = self.path_format(path.to_string());
        let read_data = crate::https::routes::add_data::AddDataFunc {}.core(
            final_path,
            data_name,
            data,
            self.api_key.clone(),
        );
        self.error_or_not(read_data.error)
    }
    pub fn delete_data<T: Display>(&self, path: T) -> Result<(), String> {
        let final_path = self.path_format(path.to_string());
        let delete_error = crate::https::routes::delete::DeleteFunc {}.main_func(final_path);
        self.error_or_not(delete_error.error)
    }
    pub fn read_data<T: Display>(&self, path: T) -> Result<Vec<String>, String> {
        let final_path = self.path_format(path.to_string());
        let read_error =
            crate::https::routes::display_data::DisplayFunc {}.core(final_path, self.null.clone());
        if read_error.error == "Success" {
            println!("Response succeeded!!!!");
            Ok(read_error.data)
        } else {
            println!("FAILED!!!!!");
            Err(read_error.error)
        }
    }
    pub fn null_write<T: Display>(&self, path: T) -> Result<(), String> {
        let final_path = self.path_format(path.to_string());
        let null_error =
            crate::https::routes::null_write::NullFunc {}.core(self.null.clone(), final_path);
        self.error_or_not(null_error.error)
    }
    pub fn add_cache_data<T: Display, T2: Display, >(
        &self,
        key: T,
        data_to_write: T2,
    ) -> Result<(), String> {
        let write_error =
            crate::https::routes::cashe::key_val_store_read::KeyFunc {}.main_func(key, data_to_write);
        self.error_or_not(write_error.error)
    }
    pub fn read_cache_data<T: Display, >(
        &self,
        key: T,
    ) -> Result<String, String> {
        let read_error =
            crate::https::routes::cashe::key_val_read::KeyFunc {}.main_func(key);
        if read_error.error != "Success" {
            Err(read_error.error)
        } else {
            Ok(read_error.data)
        }
    }
    pub fn delete_cache_data<T: Display,>(
        &self,
        key: T,
    ) -> Result<(), String> {
        let delete_error =
            crate::https::routes::cashe::key_val_delete::KeyFuncDel {}.main_func(key);
        self.error_or_not(delete_error.error)
    }
}
