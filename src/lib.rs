#![feature(proc_macro_hygiene, decl_macro)]

mod func;
pub mod functions;
mod https;
mod tests;

/// do this so nightly can be used in the code
/// this code will only work with nightly

pub struct StateData {
    pub api_key: String,
    pub null: String,
    pub data_storage_location: String,
}
