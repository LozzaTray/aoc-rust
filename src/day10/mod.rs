use crate::io;
mod sequence;

pub fn main() {
    let input: Vec<String> = io::read_to_vec("./src/day10/input.txt");
    let sequences: Vec<sequence::Sequence> = input.into_iter().map(|line| sequence::Sequence::new(line)).collect();
    let mut total_score = 0;
    for sequence in sequences {
        let score = sequence.score();
        println!("{}", score);
        total_score += score;
    }
    println!("total: {}", total_score);
}
