use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_to_vec(filename: &str) -> Vec<String> {
    let mut values: Vec<String> = vec![];
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(string_val) = line {
                values.push(string_val);
            }
        }
    }
    return values; 
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}