use std::cmp;

pub struct Board {
    D: usize,
    hits: Vec<Vec<i32>>,
    edges: Vec<Edge>
}

struct Node {
    x: usize,
    y: usize,
}

impl Node {
    pub fn new(input: &str) -> Self {
        let coords: Vec<usize> = input
            .split(",")
            .map(|val| val.parse::<usize>().unwrap())
            .collect();
        return Node {
            x: coords[0],
            y: coords[1],
        };
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

struct Edge {
    from: Node,
    to: Node,
}

impl Edge {
    pub fn nodes_between(&self) -> Vec<Node> {
        let x_delta = calc_delta(self.from.x, self.to.x);
        let y_delta = calc_delta(self.from.y, self.to.y);

        let length = cmp::max(self.to.x.abs_diff(self.from.x), self.to.y.abs_diff(self.from.y));
        let (mut x, mut y) = (self.from.x as i32, self.from.y as i32);
        let mut nodes = vec![];
        
        for _i in 0..=length {
            nodes.push(Node {x: x as usize, y: y as usize});
            x += x_delta;
            y += y_delta;
        }
        return nodes;
    }

    pub fn new(line: &str) -> Self {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let from = Node::new(parts[0]);
        let to = Node::new(parts[2]);

        return Edge { from: from, to: to };
    }
}

impl std::fmt::Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.from, self.to)
    }
}

impl Board {
    pub fn new(lines: &[String]) -> Self {
        let D = 1000;
        let mut hits: Vec<Vec<i32>> = vec![vec![0; D]; D];
        let mut edges: Vec<Edge> = vec![];

        lines.iter().for_each(|line| {
            let edge = Edge::new(line);
            for node in edge.nodes_between() {
                hits[node.x][node.y] += 1;
            }
            edges.push(edge);
        });

        return Board { D, hits, edges };
    }

    pub fn num_overlaps(&self) -> usize {
        let mut count = 0;
        for i in 0..self.D {
            for j in 0..self.D {
                if self.hits[i][j] >= 2 {
                    count += 1;
                }
            }
        }
        return count;
    }
}


fn calc_delta(start: usize, end: usize) -> i32 {
    if start == end {
        return 0;
    }
    if start < end {
        return 1;
    }
    return -1;
}
