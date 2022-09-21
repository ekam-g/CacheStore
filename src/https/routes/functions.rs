pub fn path_second(path: String, location: String) -> String {
    location + &path.replace('`', "/")
}
