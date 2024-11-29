use core::{enums::Part, file_reader::get_file_contents};

use super::shared::NiceString;

pub fn part_a() {
    let mut nice_strings: usize = 0;
    for string in get_file_contents(2015, 5, Part::A).lines().into_iter() {
        if is_nice_string(string) {
            nice_strings += 1;
        }
    }
    println!("The number of nice strings: {}", nice_strings);
}

fn is_nice_string(string: &str) -> bool {
    let nice = NiceString::parse(string);
    nice.result()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("ugknbfddgicrmopn", true)]
    #[case("aaa", true)]
    #[case("jchzalrnumimnmhp", false)]
    #[case("haegwjzuvuyypxyu", false)]
    #[case("dvszwmarrgswjxmb", false)]
    fn examples(#[case] string: &str, #[case] is_nice: bool) {
        assert_eq!(is_nice_string(string), is_nice);
    }
}
