pub fn split_path_to_name(path: &str) -> &str {
    let p = std::path::Path::new(path);
    p.file_name().unwrap().to_str().unwrap()
}