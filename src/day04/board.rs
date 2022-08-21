pub struct Board {
    D: usize,
    grid: Vec<Vec<i32>>,
    hits: Vec<Vec<bool>>
}

impl Board {
    pub fn new(lines: &[String]) -> Self {
        let D = lines.len();
        let hits = vec![vec![false; D]; D];
        let mut grid: Vec<Vec<i32>> = vec![];

        lines.iter().for_each(|line| {
            let numbers = line.split_whitespace().map(|val| val.parse::<i32>().unwrap()).collect();
            grid.push(numbers);
        });

        return Board {
            D,
            grid,
            hits
        }
    }

    pub fn process(self: &mut Self, number: i32) -> bool {
        for i in 0..self.D {
            for j in 0..self.D {
                if self.grid[i][j] == number {
                    self.hits[i][j] = true;
                    return self.is_bingo(i, j);
                }
            }
        }
        return false;
    }

    fn is_bingo(self: &Self, x: usize, y: usize) -> bool {
        let mut row_bingo = true;
        let mut col_bingo = true;

        for i in 0..self.D {
            if !self.hits[x][i] {
                row_bingo = false;
            }
            if !self.hits[i][y] {
                col_bingo = false;
            }
        }

        return row_bingo || col_bingo;
    }

    pub fn score(&self, winning_number: i32) -> i32 {
        let mut sum = 0;
        for i in 0..self.D {
            for j in 0..self.D {
                if !self.hits[i][j] {
                    sum += self.grid[i][j]; 
                }
            }
        }
        return sum * winning_number;
    }

}