#[derive(Eq, PartialEq, Copy, Clone, Hash)]
pub enum Node<'a> {
    Start,
    Small(&'a str),
    Big(&'a str),
    End
}

impl<'a> Node<'a> {
    pub fn new(val: &'a str) -> Self {
        match val {
            "start" => Node::Start,
            "end" => Node::End,
            val if val == val.to_lowercase() => Node::Small(val),
            val if val == val.to_uppercase() => Node::Big(val),
            &_ => todo!()
        }
    }
}