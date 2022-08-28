pub struct Axis {
    positions: Vec<i32>
}

impl Axis {
    pub fn new(input: &str) -> Self {
        let mut positions: Vec<i32> = input.split(",").map(|val| val.parse::<i32>().unwrap()).collect();
        positions.sort();
        return Self {
            positions
        }
    }

    pub fn min_consumption(&self) -> Result<u32, String> {
        let mut point = 0;
        let mut consumption = self.fuel_consumption(point);

        while point < 100_000_000 {
            point += 1;
            let new_consumption = self.fuel_consumption(point);

            if new_consumption > consumption {
                return Ok(consumption);
            }
            consumption = new_consumption;
        }

        return Err("Bad message".to_string());
    }

    fn fuel_consumption(&self, meeting_point: i32) -> u32 {
        return self.positions.iter().map(|val| {
            let distance = val.abs_diff(meeting_point);
            return distance * (distance + 1) / 2;
        }).sum();
    }
}