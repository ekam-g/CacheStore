pub fn path_second(path: String) -> String {
    return "database/".to_string() + &path.replace("`", "/").to_string();
}
