#![feature(proc_macro_hygiene, decl_macro)]
mod func;
mod https;

/// rustup override set nightly
/// do this so nightly can be used in the code
/// this code will only work with nightly

pub struct StateData {
    pub api_key: String,
    pub null: String,
    pub data_storage_location: String,
}

pub mod functions;
fn main() {
    StateData {
        api_key: "your_api_key".to_owned(),
        null: "null_nil_value_key:345n,234lj52".to_owned(),
        data_storage_location: "database/".to_owned(),
    }
    .start_database_online()
}

// rustup override set nightly
// do this so nightly can be used in the code
// this code will only work with nightly
