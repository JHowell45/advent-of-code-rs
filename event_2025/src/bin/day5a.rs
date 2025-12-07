use aoc_core::file_reader::get_file_contents;
use event_2025::shared::day5::FreshIngredientDB;

fn fresh_ingredient_sum(input: String) -> u32 {
    let data = input.split("\n\n").collect::<Vec<&str>>();
    let fresh_db = FreshIngredientDB::from_str(data[0]);
    data[1]
        .lines()
        .map(|l| {
            let v: u64 = l.trim().parse().unwrap();
            fresh_db.is_fresh(v) as u32
        })
        .sum()
}

fn main() {
    println!(
        "The total number of fresh ingredients is: {}",
        fresh_ingredient_sum(get_file_contents(2025, 5))
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "3-5
10-14
16-20
12-18

1
5
8
11
17
32",
        3
    )]
    fn test_fresh_ingredient_sum(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(fresh_ingredient_sum(String::from(input)), expected);
    }
}
