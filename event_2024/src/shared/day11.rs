#[derive(Debug)]
pub struct Stones {
    pub stones: Vec<String>,
}

impl Stones {
    pub fn from_string(text: &str) -> Self {
        Self {
            stones: text.split(" ").map(|s| s.to_string()).collect(),
        }
    }

    pub fn blink(&mut self) {
        let mut new_stones: Vec<String> = Vec::new();
        for stone in self.stones.iter() {
            let stone_v = stone.parse::<i32>().unwrap();
            if stone_v == 0 {
                new_stones.push(1.to_string());
            } else if stone.len() % 2 == 0 {
                new_stones.push(
                    stone[0..stone.len() / 2]
                        .parse::<i32>()
                        .unwrap()
                        .to_string(),
                );
                new_stones.push(
                    stone[stone.len() / 2..stone.len()]
                        .parse::<i32>()
                        .unwrap()
                        .to_string(),
                );
            } else {
                new_stones.push((stone_v * 2024).to_string().to_string());
            }
        }
        self.stones = new_stones;
    }

    pub fn total_stones(&self) -> usize {
        self.stones.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("125 17", vec![vec!["253000", "1", "7"], vec!["253", "0", "2024", "14168"], vec!["512072", "1", "20", "24", "28676032"] ,vec!["512", "72", "2024", "2", "0", "2", "4", "2867", "6032"] ,vec!["1036288", "7", "2", "20", "24", "4048", "1", "4048", "8096", "28", "67", "60", "32"], vec!["2097446912", "14168", "4048", "2", "0", "2", "4", "40", "48", "2024", "40", "48", "80", "96", "2", "8", "6", "7", "6", "0", "3", "2"]])]
    fn test_blink(#[case] stones: &str, #[case] blink_results: Vec<Vec<&str>>) {
        let mut stones = Stones::from_string(stones);
        for results in blink_results.iter() {
            stones.blink();
            assert_eq!(stones.stones, *results);
        }
    }
}
