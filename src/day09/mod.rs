use crate::io;
mod board;

pub fn main() {
    let input: Vec<String> = io::read_to_vec("./src/day09/input.txt");
    let board = board::Board::new(input);
    let score = board.calc_score();
    println!("{}", score);
}
