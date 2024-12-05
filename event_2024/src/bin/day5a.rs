use std::collections::{HashMap, HashSet};

fn main() {}

#[derive(Debug)]
struct LaunchSafetyManual {
    rules: PageOrderingRules,
    pages: Vec<Vec<i32>>,
}

impl LaunchSafetyManual {
    pub fn from_string(text: &str) -> Self {
        let split: Vec<&str> = text.split("\n\n").collect();
        let [page_ordering_rules, page_numbers] = [split[0], split[1]];
        Self {
            rules: PageOrderingRules::from_string(page_ordering_rules),
            pages: page_numbers
                .split('\n')
                .map(
                    |page| page.split(",").map(|n| n.parse::<i32>().unwrap()).collect()
                )
                .collect(),
        }
    }

    pub fn sum_middle_values(&self) -> i32 {
        let mut result = 0;
        for page in self.pages.iter() {
            println!("{page:?}");
            if self.validate_pages(&page) {
                println!("Valid");
                result += &page[page.len() / 2 + 1];
            }
        }
        result
    }

    fn validate_pages(&self, page_numbers: &Vec<i32>) -> bool {
        for values in page_numbers.windows(2).into_iter() {
            let (current, next) = (values[0], values[1]);
            if !self.rules.validate_next(current, next) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
struct PageOrderingRules {
    values_after: HashMap<i32, HashSet<i32>>,
}

impl PageOrderingRules {
    pub fn new() -> Self {
        Self {
            values_after: HashMap::new(),
        }
    }

    pub fn from_string(rules: &str) -> Self {
        let mut instance = Self::new();
        for rule in rules.lines().into_iter() {
            let split: Vec<&str> = rule.split("|").collect();
            let (a, b) = (
                split[0].parse::<i32>().unwrap(),
                split[1].parse::<i32>().unwrap(),
            );
            instance.add_rule(a, b);
        }
        return instance;
    }

    pub fn add_rule(&mut self, key: i32, value: i32) {
        match self.values_after.get_mut(&key) {
            Some(valid) => {
                valid.insert(value);
            }
            None => {
                self.values_after.insert(key, HashSet::from([value]));
            }
        }
    }

    pub fn validate_next(&self, current: i32, next: i32) -> bool {
        !self.values_after.contains_key(&current)
            || self.values_after.get(&current).unwrap().contains(&next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47", 143)]
    fn example(#[case] input: &str, #[case] expected: i32) {
        let manual = LaunchSafetyManual::from_string(&input);
        println!("{manual:?}");
        assert_eq!(manual.sum_middle_values(), expected);
    }
}
