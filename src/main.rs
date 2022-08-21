#![feature(proc_macro_hygiene, decl_macro)]

use crate::https::start;

pub mod https;
mod data_getting_test;


fn main() {
   start();
}
// rustup override set nightly
// do this so night can be used in the code
// this code will only work with nightly