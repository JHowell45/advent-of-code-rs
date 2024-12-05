fn main() {}

struct LaunchSafetyManual {}

impl LaunchSafetyManual {
    pub fn from_string(text: &str) -> Self {
        let split: Vec<&str> = text.split("\n\n").collect();
        let [page_ordering_rules, page_numbers] = [split[0], split[1]];
        Self {}
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
