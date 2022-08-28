mod io;
mod population;

pub fn main() {
    let input: Vec<String> = io::read_to_vec("./src/day06/input.txt");
    let mut population = population::Population::new(&input[0]);
    let mut count = population.num_fish();

    for i in 0..=256 {
        println!("day {}: {}", i, count);
        count = population.step_forward();
    }
}
