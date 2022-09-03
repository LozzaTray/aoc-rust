use std::collections::HashMap;
use crate::day12::node::Node;

pub struct Graph<'a> {
    edges: HashMap<Node<'a>, Vec<Node<'a>>>
}

impl<'a> Graph<'a> {
    pub fn new(input: &'a Vec<String>) -> Self {
        let mut edges = HashMap::new();
        input.iter().for_each(|line| {
            let parts: Vec<&str> = line.split("-").collect();
            let from = Node::new(parts[0]);
            let to = Node::new(parts[1]);
            edges.entry(from).and_modify(|v: &mut Vec<Node>| v.push(to)).or_insert(vec![to]);
            edges.entry(to).and_modify(|v: &mut Vec<Node>| v.push(from)).or_insert(vec![from]);
        });

        Self { edges }
    }

    pub fn neighbours(&self, node: &'a Node) -> &'a Vec<Node> {
        self.edges.get(node).unwrap()
    }

    pub fn num_paths_between(&self) -> usize {
        let start = Node::Start;

        let mut current_path = vec![];
        let mut all_paths = vec![];
        let mut to_visit = vec![start];

        to_visit.extend(self.neighbours(&start));

        while let Some(next) = to_visit.pop() {
            match next {
                Node::Start => continue,
                Node::End => {
                    current_path.push(Node::End);
                    all_paths.push(current_path.clone());
                    current_path.pop();
                },
                Node::Small(_val) => {
                    if current_path.contains(&next) {
                        continue;
                    }
                    current_path.push(next.clone());
                },
                Node::Big(_val) => {
                    current_path.push(next.clone());
                }
            }
            if to_visit.len() == 0 {
                if let Some(tip) = current_path.pop() {
                    to_visit.extend(self.neighbours(&tip));
                }
            }
        }

        return all_paths.len();
    }
}