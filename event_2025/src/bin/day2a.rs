use aoc_core::file_reader::get_file_contents;

fn sum_invalid_ids_from_range(id_range: &str) -> u64 {
    let mut sum: u64 = 0;
    let ids: Vec<i64> = id_range
        .split("-")
        .map(|x| x.trim().parse::<i64>().unwrap())
        .collect();
    for number in ids[0]..=ids[1] {
        let num_str: String = number.to_string();
        if num_str.chars().nth(0).unwrap() == '0' {
            sum += number as u64;
            continue;
        }
        if num_str.len() % 2 == 0 && &num_str[0..num_str.len() / 2] == &num_str[num_str.len() / 2..]
        {
            sum += number as u64;
        }
    }
    return sum;
}

fn sum_invalid_ids(ids: String) -> u64 {
    ids.split(",")
        .into_iter()
        .map(|id_range| sum_invalid_ids_from_range(id_range))
        .sum()
}

fn main() {
    println!(
        "Sum of invalid IDs: {}",
        sum_invalid_ids(get_file_contents(2025, 2))
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("11-22", 33)]
    #[case("95-115", 99)]
    #[case("998-1012", 1010)]
    #[case("1188511880-1188511890", 1188511885)]
    #[case("222220-222224", 222222)]
    #[case("1698522-1698528", 0)]
    #[case("446443-446449", 446446)]
    #[case("38593856-38593862", 38593859)]
    fn test_sum_invalid_ids_from_range(#[case] id_range: &str, #[case] expected: u64) {
        assert_eq!(sum_invalid_ids_from_range(id_range), expected);
    }

    #[rstest]
    #[case(
        "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124",
        1227775554
    )]
    fn example(#[case] input: String, #[case] expected: u64) {
        assert_eq!(sum_invalid_ids(input), expected);
    }
}
