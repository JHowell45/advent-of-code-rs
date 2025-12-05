#[derive(Debug)]
pub struct Bank {
    batteries: Vec<u8>,
}

impl Bank {
    pub fn new(batteries: &str) -> Self {
        Self {
            batteries: batteries
                .split("")
                .filter(|b| !b.is_empty())
                .map(|b| b.parse::<u8>().unwrap())
                .collect(),
        }
    }

    pub fn max_joltage(&self) -> u32 {
        let mut first_v: u8 = 0;
        let mut first_i: usize = self.batteries.len() + 100;
        for (idx, battery) in self.batteries.iter().rev().skip(1).enumerate() {
            if *battery >= first_v {
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
        return (first_v as u32 * 10) + second_v as u32;
    }

    pub fn max_joltage_n(&self, total_activated_batteries: usize) -> u64 {
        let mut max_joltage: u64 = 0;
        let total_batteries: usize = self.batteries.len();
        let mut latest_idx: usize = 0;

        for current_battery_idx in (0..total_activated_batteries).rev() {
            let mut current_v: u8 = 0;
            let mut current_index: usize = total_batteries + 100;

            println!(
                "Total: {} || Current: {} || Latest: {} || Max: {}",
                total_activated_batteries, current_index, latest_idx, current_battery_idx
            );

            for (idx, battery) in self.batteries[latest_idx..current_battery_idx]
                .iter()
                .rev()
                .enumerate()
            {
                if *battery >= current_v {
                    println!("idx: {} || v: {}", idx, battery);
                    current_v = *battery;
                    current_index = idx + 1;
                }
            }
            max_joltage += (current_v as i64 * 10_i64.pow(current_battery_idx as u32 + 1)) as u64;
            latest_idx = current_index;
            println!("Max Joltage: {} || Latest Idx: {}", max_joltage, latest_idx);
        }
        return max_joltage;
    }
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
    fn test_max_joltage_examples(#[case] batteries: &str, #[case] max_joltage: u32) {
        assert_eq!(Bank::new(batteries).max_joltage(), max_joltage);
    }

    #[rstest]
    #[case(
        "4123535244222342322334342233754335452333242522124322242423331132232242422443224231234323332243364522",
        76
    )]
    #[case(
        "7422233222252253222214221121222522222222216221232324271142242222222225251222323122427715322322522211",
        77
    )]
    #[case(
        "4136245552627274422451334432874465293326332243613632456443355732542694531343332248246255266565233636",
        99
    )]
    fn test_max_joltage_actual(#[case] batteries: &str, #[case] max_joltage: u32) {
        assert_eq!(Bank::new(batteries).max_joltage(), max_joltage);
    }

    #[rstest]
    #[case("987654321111111", 2, 98)]
    #[case("811111111111119", 2, 89)]
    #[case("234234234234278", 2, 78)]
    #[case("818181911112111", 2, 92)]
    #[case("987654321111111", 12, 987654321111)]
    #[case("811111111111119", 12, 811111111119)]
    #[case("234234234234278", 12, 434234234278)]
    #[case("818181911112111", 12, 888911112111)]
    fn test_max_joltage_n_examples(
        #[case] batteries: &str,
        #[case] n: usize,
        #[case] max_joltage: u64,
    ) {
        assert_eq!(Bank::new(batteries).max_joltage_n(n), max_joltage);
    }

    #[rstest]
    #[case(
        "4123535244222342322334342233754335452333242522124322242423331132232242422443224231234323332243364522",
        2,
        76
    )]
    #[case(
        "7422233222252253222214221121222522222222216221232324271142242222222225251222323122427715322322522211",
        2,
        77
    )]
    #[case(
        "4136245552627274422451334432874465293326332243613632456443355732542694531343332248246255266565233636",
        2,
        99
    )]
    fn test_max_joltage_n_actual(
        #[case] batteries: &str,
        #[case] n: usize,
        #[case] max_joltage: u64,
    ) {
        assert_eq!(Bank::new(batteries).max_joltage_n(2), max_joltage);
    }
}
