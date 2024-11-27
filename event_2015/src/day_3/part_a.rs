use std::collections::HashMap;

pub fn part_a() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(">", 2)]
    #[case("^>v<", 4)]
    #[case(">", 2)]
    #[case(">", 2)]
    fn example_houses_visited_count(#[case] directions: &str, #[case] visited_count: usize) {}
}
