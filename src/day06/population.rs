pub struct Population {
    fish: Vec<usize>,
}

impl Population {
    pub fn new(input: &str) -> Self {
        let mut fish = vec![0; 9];
        input.split(",").for_each(|c| {
            let age = c.parse::<usize>().unwrap();
            fish[age] += 1;
        });

        return Self { fish };
    }

    pub fn step_forward(self: &mut Self) -> usize {
        let new_spawns: usize = self.fish[0];

        for age in 1..self.fish.len() {
            self.fish[age - 1] = self.fish[age];
        }

        self.fish[8] = new_spawns;
        self.fish[6] += new_spawns;
        return self.num_fish();
    }

    pub fn num_fish(&self) -> usize {
        return self.fish.iter().sum();
    }
}

impl std::fmt::Display for Population {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.fish)
    }
}
