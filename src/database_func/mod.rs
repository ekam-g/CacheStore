use crate::func::files;

pub struct Func {}


impl Func {
    pub async fn example(&self) {
       files::WriteData {}.replace("worked".to_string(), "src/database_func/yes/cache.txt").expect("TODO: panic message");

    }
}