mod io;

pub fn main() {
    let values: Vec<String> = io::read_to_vec("./src/day03/input.txt");
    let mut readings: Vec<Vec<bool>> = vec![];
    values.iter().for_each(|val| {
        let bits: Vec<bool> = val.chars().map(|c| c == '1').collect();
        readings.push(bits);
    });

    let oxygen = filter(readings.clone(), true);
    let co2 = filter(readings.clone(), false);

    let x = to_i32(oxygen);
    let y = to_i32(co2);

    println!("{} x {} = {}", x, y, x * y);
}

fn filter(mut vals: Vec<Vec<bool>>, keep_majority: bool) -> Vec<bool> {
    let mut k = 0;

    while vals.len() > 1 {
        let count: usize = vals.iter().filter(|v| v[k]).count();
        let majority_ones = count * 2 >= vals.len();
        let to_keep = if keep_majority {majority_ones} else {!majority_ones};

        vals = vals.into_iter().filter(|v| v[k] == to_keep).collect();
        k += 1;
    }

    return vals[0].clone();
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