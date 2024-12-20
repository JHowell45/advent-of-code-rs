use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct LaunchSafetyManual {
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
                .map(|page| page.split(",").map(|n| n.parse::<i32>().unwrap()).collect())
                .collect(),
        }
    }

    pub fn sum_middle_values(&self) -> i32 {
        let mut result = 0;
        for page in self.pages.iter() {
            if self.validate_pages(&page) {
                let v = &page[page.len() / 2];
                result += v;
            }
        }
        result
    }

    pub fn fix_and_sum_middle(&self) -> i32 {
        let mut result = 0;
        for page in self.pages.iter() {
            if !self.validate_pages(&page) {
                let fixed_page = self.fix_page(&page);
                let v = &fixed_page[fixed_page.len() / 2];
                result += v;
            }
        }
        result
    }

    fn validate_pages(&self, page: &Vec<i32>) -> bool {
        let mut existing: HashSet<i32> = HashSet::new();
        for values in page.windows(2).into_iter() {
            let (current, next) = (values[0], values[1]);
            existing.insert(current);
            if !self.rules.validate_next(&existing, next) {
                return false;
            }
        }
        return true;
    }

    fn fix_page(&self, page: &Vec<i32>) -> Vec<i32> {
        let mut fixed_page = page.clone();
        let mut fixed = false;
        while !fixed {
            fixed = true;
            for idx in 0..fixed_page.len() - 1 {
                let curr = fixed_page[idx];
                let next = fixed_page[idx + 1];
                if !self.rules.validate_next(&HashSet::from([curr]), next) {
                    fixed = false;
                    fixed_page[idx] = next;
                    fixed_page[idx + 1] = curr;
                }
            }
        }
        fixed_page
    }
}

#[derive(Debug)]
struct PageOrderingRules {
    not_valid_before: HashMap<i32, HashSet<i32>>,
}

impl PageOrderingRules {
    pub fn new() -> Self {
        Self {
            not_valid_before: HashMap::new(),
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
        match self.not_valid_before.get_mut(&key) {
            Some(valid) => {
                valid.insert(value);
            }
            None => {
                self.not_valid_before.insert(key, HashSet::from([value]));
            }
        }
    }

    pub fn validate_next(&self, existing: &HashSet<i32>, next: i32) -> bool {
        if self.not_valid_before.contains_key(&next) {
            let intersection = self
                .not_valid_before
                .get(&next)
                .unwrap()
                .intersection(&existing);
            return intersection.count() == 0;
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(HashSet::from([75]), 47, true)]
    #[case(HashSet::from([75, 47]), 61, true)]
    #[case(HashSet::from([75, 47, 61]), 53, true)]
    #[case(HashSet::from([75, 47, 61, 53]), 29, true)]
    #[case(HashSet::from([97]), 61, true)]
    #[case(HashSet::from([97, 61]), 53, true)]
    #[case(HashSet::from([97, 61, 53]), 29, true)]
    #[case(HashSet::from([97, 61, 53, 29]), 13, true)]
    #[case(HashSet::from([75]), 29, true)]
    #[case(HashSet::from([75, 29]), 13, true)]
    #[case(HashSet::from([75]), 97, false)]
    #[case(HashSet::from([61]), 13, true)]
    #[case(HashSet::from([61, 13]), 29, false)]
    fn test_rules(#[case] existing: HashSet<i32>, #[case] next: i32, #[case] valid: bool) {
        let rules = PageOrderingRules::from_string("47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13\n97|29\n53|29\n61|53\n97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13");
        assert_eq!(rules.validate_next(&existing, next), valid);
    }
}
