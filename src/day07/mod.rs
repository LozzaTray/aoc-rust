mod io;
mod axis;

pub fn main() {
    let input: Vec<String> = io::read_to_vec("./src/day07/input.txt");
    let axis = axis::Axis::new(&input[0]);
    let consumption = axis.min_consumption();
    println!("{}", consumption.unwrap());
}
