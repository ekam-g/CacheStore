#![feature(proc_macro_hygiene, decl_macro)]

mod database_func;
mod func;
mod https;

/// rustup override set nightly
/// do this so nightly can be used in the code
/// this code will only work with nightly

pub fn start() {
    https::Web {}.start();
}
