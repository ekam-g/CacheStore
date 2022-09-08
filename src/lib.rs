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
    pub fn read(self){
        
    }
}
