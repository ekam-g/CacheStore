#![feature(proc_macro_hygiene, decl_macro)]
#![feature(io_error_more)]

mod https;
mod database_func;
mod func;

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        database_func::Func {}.example().await;
     });
    https::Web{}.start();
}
// rustup override set nightly
// do this so night can be used in the code
// this code will only work with nightly