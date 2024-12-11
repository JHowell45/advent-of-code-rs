pub struct Stones {
    pub stones: Vec<i32>,
}

impl Stones {
    pub fn from_string(text: &str) -> Self {
        Self {
            stones: text
                .split(" ")
                .map(|s| s.to_string().parse::<i32>().unwrap())
                .collect(),
        }
    }

    pub fn blink(&mut self) {
        for stone in self.stones.iter_mut() {
            if *stone == 0 {
                *stone = 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_blink() {

    }
}