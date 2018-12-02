use std::str::FromStr;
use std::{fs::File, io::BufRead, io::BufReader};

pub fn map_lines_to_int32(path: &str) -> Vec<i32> {
    let file = File::open(path).unwrap();
    let lines = BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>();

    lines.iter().map(|s| i32::from_str(s).unwrap()).collect()
}
