use crate::io;
mod graph;
mod node;

pub fn main() {
    let input: Vec<String> = io::read_to_vec("./src/day12/example.txt");
    let graph = graph::Graph::new(&input);

    let num_paths = graph.num_paths_between();
    println!("Num paths: {}", num_paths);
}
