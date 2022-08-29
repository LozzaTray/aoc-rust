pub struct Board {
    heights: Vec<Vec<u32>>,
    n: usize,
    m: usize
}

impl Board {
    pub fn new(lines: Vec<String>) -> Self {
        let heights: Vec<Vec<u32>> = lines
            .iter()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();
        let n = heights.len();
        let m = heights[0].len();
        Self {
            heights,
            n,
            m
        }
    }

    pub fn calc_score(&self) -> usize {
        let mut basin_sizes = vec![];
        for i in 0..self.n {
            for j in 0..self.m {
                if self.is_minimum(i, j) {
                    let height = self.heights[i][j];
                    let basin_size = self.basin_size(i, j);
                    println!("({}, {}): {} - {}", i, j, height, basin_size);
                    basin_sizes.push(basin_size);
                }
            }
        }
        
        basin_sizes.sort();
        println!("{:?}", basin_sizes);
        let score = basin_sizes.into_iter().rev().take(3).reduce(|left, right| left * right).unwrap();
        return score;
    }

    fn neighbours(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        let mut positions_to_check = vec![];
        if i > 0 {
            positions_to_check.push((i - 1, j));
        }
        if i < self.n - 1 {
            positions_to_check.push((i + 1, j));
        }
        if j > 0 {
            positions_to_check.push((i, j - 1));
        }
        if j < self.m - 1 {
            positions_to_check.push((i, j + 1));
        }
        return positions_to_check;
    }

    fn is_minimum(&self, i: usize, j: usize) -> bool {
        let positions_to_check = self.neighbours(i, j);
        let val = self.heights[i][j];

        return positions_to_check.iter().all(|pos| {
            val < self.heights[pos.0][pos.1]
        })
    }

    fn basin_size(&self, i: usize, j: usize) -> usize {
        let mut size_of_basin = 0;
        let mut in_basin = vec![vec![false; self.m]; self.n];
        let mut to_check = vec![(i, j)];

        while let Some(pos) = to_check.pop() {
            let height = self.heights[pos.0][pos.1];
            //println!("{:?} - {}", pos, height);
            in_basin[pos.0][pos.1] = true;
            size_of_basin += 1;

            let next_neighbours: Vec<(usize, usize)> = self.neighbours(pos.0, pos.1).into_iter().filter(|next| {
                let next_height = self.heights[next.0][next.1];
                if next_height == 9 || next_height <= height || in_basin[next.0][next.1] {
                    return false;
                }
                return true;
            }).collect();
            to_check.extend(next_neighbours);
        }

        // let mut line = "".to_string();
        // for a in 0..cmp::max(i + 20, self.n) {
        //     for b in 0..cmp::max(j + 20, self.m) {
        //         if in_basin[a][b] {
        //             line = line + &(self.heights[a][b].to_string() + "* ");
        //         } else {
        //             line = line + &(self.heights[a][b].to_string() + "  ");
        //         }
        //     }
        //     println!("{}", line);
        // }

        // for line in in_basin {
        //     println!("{:?}", line.iter().map(|b| {
        //         if *b {
        //             return "1".to_owned();
        //         }
        //         return "0".to_owned();
        //     }).reduce(|a, b| a + b.as_str()).unwrap());
        // }

        size_of_basin
    }
}
