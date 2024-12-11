use std::collections::HashMap;

#[derive(Debug)]
pub struct Stones {
    pub stones: Vec<u64>,
    stones_cache: HashMap<u64, usize>,
}

impl Stones {
    pub fn from_string(text: &str) -> Self {
        Self {
            stones: text
                .split(" ")
                .map(|s| s.to_string().parse::<u64>().unwrap())
                .collect(),
            stones_cache: HashMap::new(),
        }
    }

    pub fn total_stones(&mut self, blinks: usize) -> usize {
        let mut stones_count: usize = 0;
        for idx in 0..self.stones.len() {
            let stone = self.stones.get(idx).unwrap();
            match self.stones_cache.get(stone) {
                Some(v) => stones_count += v,
                None => {
                    let local_count: usize = self.blink(stone.clone(), blinks);
                    stones_count += local_count;
                }
            }
        }
        return stones_count;
    }

    fn blink(&mut self, v: u64, n: usize) -> usize {
        if n == 0 {
            return 1;
        }

        let stone_l: u32 = v.checked_ilog10().unwrap_or(0) + 1;
        if v == 0 {
            return self.blink(1, n - 1);
        } else if stone_l % 2 == 0 {
            let splitter: u64 = 10_u64.pow(stone_l / 2);
            let first = v / splitter;
            let second = v - (first * splitter);

            return self.blink(first, n - 1) + self.blink(second, n - 1);
        } else {
            return self.blink(v * 2024, n - 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // #[rstest]
    // #[case("125 17", vec![vec![253000, 1, 7], vec![253, 0, 2024, 14168], vec![512072, 1, 20, 24, 28676032] ,vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032] ,vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32], vec![2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6, 0, 3, 2]])]
    // fn test_blink(#[case] stones: &str, #[case] blink_results: Vec<Vec<u64>>) {
    //     let mut stones = Stones::from_string(stones);
    //     for results in blink_results.iter() {
    //         stones.blink();
    //         assert_eq!(stones.stones, *results);
    //     }
    // }

    #[rstest]
    #[case("125 17", 6, 22)]
    #[case("125 17", 25, 55312)]
    fn test_total_stones(
        #[case] starting_stones: &str,
        #[case] blinks: usize,
        #[case] total: usize,
    ) {
        let mut stones = Stones::from_string(starting_stones);
        assert_eq!(stones.total_stones(blinks), total);
    }

    // #[rstest]
    // #[case("0 1 10 99 999", vec![1, 2024, 1, 0, 9, 9, 2021976])]
    // fn test_stones(#[case] input: &str, #[case] expected: Vec<u64>) {
    //     let mut stones = Stones::from_string(input);
    //     println!();
    //     stones.blink();
    //     assert_eq!(stones.stones, expected);
    // }
}
