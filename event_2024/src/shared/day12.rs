pub struct Region {
    name: String,
    area: usize,
    perimeter: usize
}

pub struct Farm {
    plots: Vec<Vec<char>>
}

impl Farm {
    pub fn from_string(puzzle: &str) -> Self {
        Self {
            plots: puzzle.split("\n").map(|r| r.chars().collect()).collect()
        }
    }

    pub fn build_regions(&self) -> Vec<Region> {
        let regions: Vec<Region> = Vec::new();
        return regions;
    }

    fn plot_same_cardinals(&self, y: usize, x: usize, max_y: usize, max_x: usize) -> usize {
        let mut same_count: usize = 0;
        let plot = self.plots[y][x];
        if y > 0 && plot == self.plots[y - 1][x] {
            same_count += 1;
        }
        if y < max_y - 1 && plot == self.plots[y + 1][x] {
            same_count += 1;
        }
        if x > 0 && plot == self.plots[y][x - 1] {
            same_count += 1;
        }
        if x < max_x - 1 && plot == self.plots[y][x + 1] {
            same_count += 1;
        }
        return same_count;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn example() {}
}