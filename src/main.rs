#![feature(proc_macro_hygiene, decl_macro)]

mod func;
mod https;

fn main() {
    https::Web {}.start();
}

// rustup override set nightly
// do this so nightly can be used in the code
// this code will only work with nightly
