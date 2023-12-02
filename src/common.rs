use std::fs;
use std::path::Path;

pub fn load_from(filename: &str) -> String {
    let path = format!("data{}{}", std::path::MAIN_SEPARATOR, filename);
    let data_file = Path::new(path.as_str());
    fs::read_to_string(data_file).unwrap()
}
