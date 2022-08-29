use crate::io;
mod board;

pub fn main() {
    let input: Vec<String> = io::read_to_vec("./src/day11/input.txt");
    let mut board = board::Board::new(input);

    let mut total_flashes = 0;
    for i in 1..=1_000_000 {
        board.print();
        let num_flashes = board.step_forward();
        println!("\nstep {} - {}\n\n", i, num_flashes);
        total_flashes += num_flashes;

        if num_flashes == 100 {
            println!("All flashed at step: {}", i);
            break;
        }
    }

    println!("Total - {}", total_flashes);
}
