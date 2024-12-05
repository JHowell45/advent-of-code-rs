use std::collections::{HashMap, HashSet};

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
            rules: PageOrderingRules::from_string(page_ordering_rules),
            page_numbers: page_numbers.split(',').map(|n| n.parse::<i32>().unwrap()).collect()
        }
    }
}

struct PageOrderingRules {
    values_after: HashMap<i32, HashSet<i32>>
}

impl PageOrderingRules {
    pub fn new() -> Self {
        Self {
            values_after: HashMap::new()
        }
    }

    pub fn from_string(rules: &str) -> Self {
        let mut instance = Self::new();
        for rule in rules.lines().into_iter() {
            let split: Vec<&str> = rule.split("|").collect();
            let (a, b) = (split[0].parse::<i32>().unwrap(), split[0].parse::<i32>().unwrap());
            instance.add_rule(a, b);
        }
        return instance;
    }

    pub fn add_rule(&mut self, key: i32, value: i32) {
        match self.values_after.get_mut(key) {
            Some(valid) => {
                valid.insert(value);
            }
            None => {
                self.values_after.insert(key, HashSet::from([value]));
            }
        }
    }

    pub fn validate_next(&self, current: i32, next: i32) -> bool {
        self.values_after.contains_key(&current) || self.values_after.get(&current).unwrap().contains(&next)
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn example() {}
}
