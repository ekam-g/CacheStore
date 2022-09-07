#![feature(proc_macro_hygiene, decl_macro)]

mod func;
mod https;

pub struct StateData {
    api_key: String,
    null: String,
}

fn main() {
    let init_val = StateData {
        api_key: "your_api_key".to_string(), //TODO when using this please write an api key
        null: "null_nil_value_key:345n,234lj52".to_string(),
    };
    https::Web {}.start(init_val);
}

// rustup override set nightly
// do this so nightly can be used in the code
// this code will only work with nightly
