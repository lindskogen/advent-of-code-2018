use std::io::Read;
use std::str::FromStr;
use std::{fs::File, io::BufRead, io::BufReader};

pub fn map_lines_to_int32(path: &str) -> Vec<i32> {
    let lines = map_lines_to_strings(path);
    lines.iter().map(|s| i32::from_str(s).unwrap()).collect()
}

pub fn map_lines_to_strings(path: &str) -> Vec<String> {
    let file = File::open(path).unwrap();
    BufReader::new(file).lines().map(|l| l.unwrap()).collect()
}

pub fn read_file_to_string(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut result_string = String::new();
    file.read_to_string(&mut result_string);
    result_string
}
