#![feature(proc_macro_hygiene, decl_macro)]

mod func;
mod https;

/// rustup override set nightly
/// do this so nightly can be used in the code
/// this code will only work with nightly

pub struct StateData {
    api_key: String,
    null: String,
    data_storage_location: String,
}

impl StateData {
    pub fn start_database_online(self) {
        https::Web {}.start(self);
    }
    pub fn write_data(&self, data: String, path: String, data_name: String) -> Result<(), String> {
        let final_path: String = format!("{}{}", self.data_storage_location, path);
        drop(path);
        let read_data = https::routes::add_data::AddDataFunc {}.core(
            final_path,
            data_name,
            data,
            self.api_key.to_string(),
        );
        if read_data.error == "Success" {
            Ok(())
        } else {
            Err(read_data.error)
        }
    }
    
}
