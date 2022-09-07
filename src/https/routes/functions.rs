pub fn path_second(path: String, location: String) -> String {
    return location + &path.replace("`", "/").to_string();
}
