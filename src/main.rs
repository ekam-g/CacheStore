#![feature(proc_macro_hygiene, decl_macro)]

mod func;
mod https;

pub struct StateData {
    api_key: String,
    null: String,
    data_storage_location: String,
}

impl StateData {
    pub fn start_database_online(self) {
        https::Web {}.start(self);
    }
}

fn main() {
    StateData {
        api_key: "your_api_key".to_string(),
        null: "null_nil_value_key:345n,234lj52".to_string(),
        data_storage_location: "database/".to_string(),
    }
    .start_database_online()
}

// rustup override set nightly
// do this so nightly can be used in the code
// this code will only work with nightly
