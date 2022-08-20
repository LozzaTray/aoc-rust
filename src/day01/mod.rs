use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    let mut values: Vec<i32> = vec![];

    if let Ok(lines) = read_lines("./src/day01/input.txt") {
        for line in lines {
            if let Ok(string_val) = line {
                let val = string_val.parse::<i32>().unwrap();
                values.push(val);
            }
        }
    }
    println!("Window size: {}, {}", 1, num_increased(&values, 1));
    println!("Window size: {}, {}", 3, num_increased(&values, 3));
    return 
}

fn num_increased(values: &Vec<i32>, window: usize) -> i32 {
    let mut counter = 0;

    for i in 0..values.len()-window {
        if values[i] < values[i+window] {
            counter += 1;
        }
    }

    return counter;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}