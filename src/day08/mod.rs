mod io;
mod panel;

pub fn main() {
    let input: Vec<String> = io::read_to_vec("./src/day08/input.txt");
    let panels: Vec<panel::Panel> = input.iter().map(|line| panel::Panel::new(line)).collect();

    let score: usize = panels.iter().map(|p| p.num_known_digits_in_output()).sum();
    println!("{}", score);
}
