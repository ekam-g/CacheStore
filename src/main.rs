#![feature(proc_macro_hygiene, decl_macro)]
//this is needed don't delete
use crate::https::start;

pub mod https;


fn main() {
   start();
}
// rustup override set nightly
// do this so night can be used in the code
// this code will only work with nightly