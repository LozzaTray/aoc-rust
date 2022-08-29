const D: usize = 7;

struct Key {
    mapping: [usize; D]
}

struct Digit {
    segments: [bool; D],
}

impl Digit {
    pub fn new(group: &str) -> Self {
        let alphabet = "abcdefg";
        let mut segments = [false; D];

        group.chars().for_each(|c| {
            let index = alphabet.find(c).unwrap();
            segments[index] = true;
        });

        return Self { segments };
    }

    pub fn num_segments_activated(&self) -> usize {
        return self.segments.iter().filter(|s| **s).count();
    }

    pub fn translate(&self, key: Key) -> Result<usize, String> {
        let mut translated_segments = [false; D];

        for i in 0..D {
            let mapped = key.mapping[i];
            translated_segments[mapped] = self.segments[i];
        }

        let translated = Digit { segments: translated_segments };
        let trials_by_size: [Vec<usize>; D + 1] = [vec![], vec![], vec![1], vec![7], vec![4], vec![2, 3, 5], vec![0, 6, 9], vec![8]];

        let trials = trials_by_size[translated.num_segments_activated()];

        for trial in trials {
            if Self::from_num(trial) == translated {
                return Ok(trial);
            }
        }

        return Err("Invalid 7-seg".to_string());
    }

    pub fn from_num(num: usize) -> Self {
        match num {
            1 => Self::new("cf"),
            7 => Self::new("acf"),
            4 => Self::new("bcdf"),
            2 => Self::new("acdeg"),
            3 => Self::new("acdfg"),
            5 => Self::new("abdfg"),
            0 => Self::new("abcefg"),
            6 => Self::new("abdefg"),
            9 => Self::new("abcdfg"),
            8 => Self::new("abcdefg"),
            default => panic!("Bad argument")
        }
    }
}

impl PartialEq for Digit {
    fn eq(&self, other: &Self) -> bool {
        (0..D).all(|i| self.segments[i] == other.segments[i])
    }
}
impl Eq for Digit {}

pub struct Panel {
    inputs: Vec<Digit>,
    outputs: Vec<Digit>,
}

impl Panel {
    pub fn new(line: &str) -> Self {
        let components: Vec<&str> = line.split("|").collect();

        let inputs = components[0]
            .split_whitespace()
            .map(|block| Digit::new(block))
            .collect();
        let outputs = components[1]
            .split_whitespace()
            .map(|block| Digit::new(block))
            .collect();

        return Self { inputs, outputs };
    }

    pub fn num_known_digits_in_output(&self) -> usize {
        let unique_counts: [usize; 4] = [2, 4, 3, 7];
        return self
            .outputs
            .iter()
            .map(|digit| {
                let num_segments = digit.num_segments_activated();
                // println!("digit: {}", num_segments);
                if unique_counts.contains(&num_segments) {
                    return 1;
                }
                return 0;
            })
            .sum();
    }

    pub fn find_decryptor(&self) -> [usize; D] {
        let key = [0; D];

        let one;

        for i in 0..self.inputs.len() {
            let input = &self.inputs[i];
            let num_segments = input.num_segments_activated();

            if num_segments == 2 {
                //mut one = input;
            }
        }
        for input in self.inputs.into_iter() {

        }

        return key;
    }
}
