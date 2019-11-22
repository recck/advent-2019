use std::fs;

pub fn read(file_path: String) -> String {
    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");

    return contents;
}

pub fn exists(file_path: String) -> bool {
    return fs::metadata(file_path).is_ok();
}