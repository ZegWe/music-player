pub fn split_path_to_name(path: &str) -> &str {
    let str = path.split("\\").collect::<Vec<&str>>();
    str.last().unwrap()
}