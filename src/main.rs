#![feature(proc_macro_hygiene, decl_macro)]
#![allow(dead_code)]

mod database_func;
mod func;
mod https;

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        database_func::Func {}.example().await;
        // use this if you want to spawn aync funtions that don't modify https funtions
    });
    https::Web {}.start();
}

// rustup override set nightly
// do this so nightly can be used in the code
// this code will only work with nightly
