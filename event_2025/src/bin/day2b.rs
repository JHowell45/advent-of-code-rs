use aoc_core::file_reader::get_file_contents;

fn check_number_repetitions(number: &str) -> bool {
    let mut split: usize = (number.len() / 2) + 1;
    while split > 0 {
        let mut curr: Option<&str> = None;
        if number.len() != split && number.len() % split == 0 {
            let mut safe: bool = true;
            for n in 0..number.len() / split {
                let a: usize = split * n;
                let b: usize = split * (n + 1);
                let chunk = &number[a..b];
                // println!("Number: {} || Split: {} || [a: {}, b: {}] || Chunk: {} || Curr: {:?}", number, split, a, b, chunk, curr);
                match curr {
                    None => curr = Some(chunk),
                    Some(c) => {
                        if c != chunk {
                            safe = false;
                            break;
                        }
                    }
                }
            }
            if safe {
                return true;
            }
        }
        split -= 1;
    }
    return false;
}

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
        } else if check_number_repetitions(num_str.as_str()) {
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
    #[case("11", true)]
    #[case("12", false)]
    #[case("13", false)]
    #[case("14", false)]
    #[case("15", false)]
    #[case("16", false)]
    #[case("22", true)]
    #[case("99", true)]
    #[case("111", true)]
    #[case("1010", true)]
    #[case("123123", true)]
    #[case("222222", true)]
    #[case("123123123", true)]
    #[case("1188511885", true)]
    #[case("1188511886", false)]
    #[case("1188511884", false)]
    fn test_check_number_repetitions(#[case] value: &str, #[case] expected: bool) {
        assert_eq!(check_number_repetitions(value), expected);
    }

    #[rstest]
    #[case("11-22", 33)]
    #[case("95-115", 210)]
    #[case("998-1012", 2009)]
    #[case("1188511880-1188511890", 1188511885)]
    #[case("222220-222224", 222222)]
    #[case("1698522-1698528", 0)]
    #[case("446443-446449", 446446)]
    #[case("38593856-38593862", 38593859)]
    #[case("565653-565659", 565656)]
    #[case("824824821-824824827", 824824824)]
    #[case("2121212118-2121212124", 2121212121)]
    fn test_sum_invalid_ids_from_range(#[case] id_range: &str, #[case] expected: u64) {
        assert_eq!(sum_invalid_ids_from_range(id_range), expected);
    }

    #[rstest]
    #[case(
        "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124",
        4174379265
    )]
    fn example(#[case] input: String, #[case] expected: u64) {
        assert_eq!(sum_invalid_ids(input), expected);
    }
}
