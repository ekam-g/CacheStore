#![feature(proc_macro_hygiene, decl_macro)]

mod https;
mod data_getting_test;
mod files;

#[tokio::main]
async fn main() {
    tokio::spawn(async {
         data_getting_test::Data {}.get().await;
     });
    https::Web{}.start();
}
// rustup override set nightly
// do this so night can be used in the code
// this code will only work with nightly