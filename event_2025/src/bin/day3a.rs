use aoc_core::file_reader::get_file_contents;

#[derive(Debug)]
struct Bank {
    batteries: Vec<u8>,
}

impl Bank {
    fn new(batteries: &str) -> Self {
        Self {
            batteries: batteries
                .split("")
                .filter(|b| !b.is_empty())
                .map(|b| b.parse::<u8>().unwrap())
                .collect(),
        };
    }

    fn max_joltage(&self) -> u32 {
        let mut first_v: u8 = 0;
        let mut first_i: usize = self.batteries.len() + 100;
        for (idx, battery) in self.batteries.iter().rev().skip(1).enumerate() {
            if *battery > first_v {
                first_v = *battery;
                first_i = self.batteries.len() - 1 - idx;
            }
        }
        let mut second_v: u8 = 0;
        for battery in self.batteries[first_i..].iter().rev() {
            if battery > &second_v {
                second_v = *battery;
            }
        }
        let max = (first_v.to_string() + second_v.to_string().as_str())
            .parse::<u32>()
            .unwrap();
        println!("{:?} || {}", self.batteries, max);
        return max;
    }
}

fn maximum_bank_joltage_sum(banks: String) -> u32 {
    banks
        .lines()
        .into_iter()
        .map(|batteries| Bank::new(batteries).max_joltage())
        .sum()
}

fn main() {
    println!(
        "Sum of the maximum voltage for each bank: {}",
        maximum_bank_joltage_sum(get_file_contents(2025, 3))
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("987654321111111", 98)]
    #[case("811111111111119", 89)]
    #[case("234234234234278", 78)]
    #[case("818181911112111", 92)]
    fn test_(#[case] batteries: &str, #[case] max_joltage: u32) {
        assert_eq!(Bank::new(batteries).max_joltage(), max_joltage);
    }

    #[rstest]
    #[case(
        "987654321111111
811111111111119
234234234234278
818181911112111",
        357
    )]
    fn example(#[case] banks: &str, #[case] maximum_joltage_sum: u32) {}
}
