#![feature(proc_macro_hygiene, decl_macro)]

use std::thread;

mod https;
mod data_getting_test;
mod files;

fn main() {
    thread::spawn(|| {
        data_getting_test::Data {}.get()
    });
    https::Web{}.start();

}
// rustup override set nightly
// do this so night can be used in the code
// this code will only work with nightly