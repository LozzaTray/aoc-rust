pub struct Sequence {
    vals: Vec<Brace>
}

#[derive(PartialEq)]
enum Type {
    Basic,  // ()
    Square, // []
    Curly,  // {}
    Angle   // <>
}

enum Brace {
    Open(Type),
    Close(Type)
}

impl Sequence {
    pub fn new(line: String) -> Self {
        let vals = line.chars().map(|c| {
            match c {
                '(' => Brace::Open(Type::Basic),
                '[' => Brace::Open(Type::Square),
                '{' => Brace::Open(Type::Curly),
                '<' => Brace::Open(Type::Angle),
                ')' => Brace::Close(Type::Basic),
                ']' => Brace::Close(Type::Square),
                '}' => Brace::Close(Type::Curly),
                '>' => Brace::Close(Type::Angle),
                other => panic!("Unexpected char: {}", other)
            }
        }).collect();
        Self { vals }
    }

    pub fn score(&self) -> u32 {
        let mut stack: Vec<&Type> = vec![];

        for i in 0..self.vals.len() {
            let val = &self.vals[i];
            match val {
                Brace::Open(t) => stack.push(t),
                Brace::Close(t) => {
                    let expected = stack.pop().unwrap();
                    if t != expected {
                        return score_for_type(t);
                    }
                }
            }
        }
        return 0;
    }
}

fn score_for_type(t: &Type) -> u32 {
    match t {
        Type::Basic => 3,
        Type::Square => 57,
        Type::Curly => 1197,
        Type::Angle => 25137
    }
}