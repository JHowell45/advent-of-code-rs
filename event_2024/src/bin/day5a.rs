fn main() {}

struct LaunchSafetyManual {
    rules: PageOrderingRules,
    page_numbers: Vec<i32>
}

impl LaunchSafetyManual {
    pub fn from_string(text: &str) -> Self {
        let split: Vec<&str> = text.split("\n\n").collect();
        let [page_ordering_rules, page_numbers] = [split[0], split[1]];
        Self {
            rules: PageOrderingRules::new(),
            page_numbers: page_numbers.split(',').map(|n| n.parse::<i32>().unwrap()).collect()
        }
    }
}

struct PageOrderingRules {}

impl PageOrderingRules {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn example() {}
}
