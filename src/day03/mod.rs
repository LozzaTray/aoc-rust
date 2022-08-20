mod io;

pub fn main() {
    let values: Vec<String> = io::read_to_vec("./src/day03/input.txt");

    let num_readings = values.len();
    let num_bits = values[0].len();

    let mut bit_counts = vec![0; num_bits];
    values.iter().for_each(|val| {
        let bits: Vec<char> = val.chars().collect();

        for i in 0..num_bits {
            if bits[i] == '1' {
                bit_counts[i] += 1;
            }
        }
    });

    let majority_votes: Vec<bool> = bit_counts.iter().map(|count| count * 2 >= num_readings).collect();
    let flipped: Vec<bool> = majority_votes.iter().map(|x| !x).collect();

    let x = to_i32(majority_votes);
    let y = to_i32(flipped);

    println!("{} x {} = {}", x, y, x * y);
}

fn to_i32(boolean_vec: Vec<bool>) -> i32 {
    let mut cum = 0;
    let mut power = 1;

    for i in (0..boolean_vec.len()).rev() {
        if boolean_vec[i] {
            cum += power;
        }
        power *= 2;
    }
    cum
}