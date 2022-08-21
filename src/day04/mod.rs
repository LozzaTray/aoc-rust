mod io;
mod board;

pub fn main() {
    let input: Vec<String> = io::read_to_vec("./src/day04/input.txt");
    let number_sequence: Vec<i32> = input[0].split(",").map(|c| c.parse::<i32>().unwrap()).collect();

    let mut bingo_cards = vec![];
    for i in (2..input.len()).step_by(6) {
        let card = board::Board::new(&input[i..i+5]);
        bingo_cards.push(card);
    }

    for num in number_sequence {
        let mut to_delete = vec![];
        let mut i = 0;
        for card in &mut bingo_cards {
            let is_bingo = card.process(num);
            if is_bingo {
                let score = card.score(num);
                println!("Winning score: {}", score);
                to_delete.push(i);
            }
            i += 1;
        }
        to_delete.reverse();
        for del in to_delete {
            bingo_cards.remove(del);
        }
        if bingo_cards.len() == 0 {
            return;
        }
    }
}
