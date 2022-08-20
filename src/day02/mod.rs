use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::fmt;

pub fn main() {
    let mut pos = Position {distance: 0, depth: 0, aim: 0};
    let instructions = read_instructions();

    println!("{}", pos);
    instructions.iter().for_each(|i| handle_instruction(i, &mut pos));
    println!("{}", pos);

    println!("Multiplied: {}", pos.distance * pos.depth);
}

struct Position {
    distance: i32,
    depth: i32,
    aim: i32
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(distance: {}, depth: {}, aim: {})", self.distance, self.depth, self.aim)
    }
}

fn handle_instruction(instruction: &Instruction, pos: &mut Position) {
    match instruction {
        Instruction::Forwards(x) => {
            pos.distance += x;
            pos.depth += x * pos.aim;
        },
        Instruction::Aim(a) => pos.aim += a
    }
}

enum Instruction {
    Forwards(i32),
    Aim(i32)
}

fn read_instructions() -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = vec![];

    if let Ok(lines) = read_lines("./src/day02/input.txt") {
        for line in lines {
            if let Ok(string_val) = line {
                let instruction = parse_instruction(&string_val);
                instructions.push(instruction);
            }
        }
    }

    return instructions;
}

fn parse_instruction(line: &str) -> Instruction {
    let components = line.split(" ").collect::<Vec<&str>>();

    let action = components[0];
    let distance = components[1].parse::<i32>().unwrap();

    if action == "forward" {
        return Instruction::Forwards(distance);
    }
    if action == "down" {
        return Instruction::Aim(distance);
    }
    if action == "up" {
        return Instruction::Aim(-distance);
    }
    panic!("Unable to parse instruction");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}