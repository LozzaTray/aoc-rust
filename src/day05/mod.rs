mod io;
mod board;

pub fn main() {
    let input: Vec<String> = io::read_to_vec("./src/day05/input.txt");
    let board = board::Board::new(&input);
    let overlaps = board.num_overlaps();
    println!("{}", overlaps);
}
