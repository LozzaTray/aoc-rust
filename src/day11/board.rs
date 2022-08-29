use std::collections::VecDeque;

const MAX_ENERGY: u32 = 9;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

pub struct Board<T> {
    vals: Vec<Vec<T>>,
    n: usize,
    m: usize,
}

impl<T: ::core::marker::Copy> Board<T> {
    fn neighbours(&self, point: &Point) -> Vec<Point> {
        let (i, j) = (point.x as i32, point.y as i32);
        let positions_to_check = vec![
            (i - 1, j - 1),
            (i - 1, j),
            (i - 1, j + 1),
            (i, j - 1),
            (i, j + 1),
            (i + 1, j - 1),
            (i + 1, j),
            (i + 1, j + 1),
        ];

        return positions_to_check
            .into_iter()
            .filter(|p| {
                if p.0 >= 0 && p.0 < self.n as i32 && p.1 >= 0 && p.1 < self.m as i32 {
                    return true;
                }
                return false;
            })
            .map(|p| Point {
                x: p.0 as usize,
                y: p.1 as usize,
            })
            .collect();
    }

    fn get(&self, p: &Point) -> T {
        return self.vals[p.x][p.y];
    }

    fn set(&mut self, p: &Point, val: T) {
        self.vals[p.x][p.y] = val;
    }
}

impl Board<u32> {
    pub fn new(lines: Vec<String>) -> Self {
        let vals: Vec<Vec<u32>> = lines
            .iter()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();
        let n = vals.len();
        let m = vals[0].len();
        Self { vals, n, m }
    }

    fn increment(&mut self, p: &Point) -> u32 {
        let original = self.get(p);
        let updated = original + 1;
        self.set(p, updated);
        return updated;
    }

    pub fn step_forward(&mut self) -> u32 {
        let flashers_values = vec![vec![false; self.m]; self.n];
        let mut has_flashed = Board {
            vals: flashers_values,
            n: self.n,
            m: self.m,
        };

        let mut to_check = VecDeque::from(self.all_points());
        while let Some(p) = to_check.pop_front() {
            //println!("{}", to_check.len());
            let new_val = self.increment(&p);
            if has_flashed.get(&p) {
                continue;
            }

            if new_val > MAX_ENERGY {
                has_flashed.set(&p, true);
                let neighbours = self.neighbours(&p);
                neighbours.into_iter()
                    .for_each(|point| to_check.push_back(point));
            }
            //self.print();
        }

        let mut num_flashes = 0;
        self.all_points().iter().for_each(|p| {
            if has_flashed.get(p) {
                num_flashes += 1;
                self.set(p, 0);
            }
        });

        return num_flashes;
    }

    fn all_points(&self) -> Vec<Point> {
        let mut points = vec![];
        for i in 0..self.n {
            for j in 0..self.m {
                let p = Point { x: i, y: j };
                points.push(p);
            }
        }
        return points;
    }

    pub fn print(&self) {
        for i in 0..self.n {
            let mut line = "".to_string();
            for j in 0..self.m {
                let point = Point {x: i, y: j};
                line = line + &self.get(&point).to_string();
            }
            println!("{}", line);
        }
    }
}
