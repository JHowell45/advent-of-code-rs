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
                new_stones.push(stone[0..stone.len() / 2].to_string());
                new_stones.push(stone[stone.len() / 2..stone.len()].to_string());
            } else {
                new_stones.push((stone_v * 2024).to_string().to_string());
            }
        }
        self.stones = new_stones;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_blink() {}
}
