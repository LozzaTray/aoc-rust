use crate::io;
mod sequence;

pub fn main() {
    let input: Vec<String> = io::read_to_vec("./src/day10/input.txt");
    let sequences: Vec<sequence::Sequence> = input
        .into_iter()
        .map(|line| sequence::Sequence::new(line))
        .collect();

    let mut autocomplete_scores: Vec<u64> = sequences
        .into_iter()
        .filter(|s| s.badness_score() == 0)
        .map(|s| s.autocomplete_score())
        .collect();
    autocomplete_scores.sort();
    println!("{:?}", autocomplete_scores);
    let median_score = autocomplete_scores[(autocomplete_scores.len() - 1) / 2];

    println!("median: {}", median_score);
}
