use core::file_reader::get_file_contents;

use event_2024::shared::day4::WordSearch;

fn main() {
    let word: &str = "XMAS";
    let word_search = WordSearch::from_string(get_file_contents(2024, 4).as_str());
    word_search.display_search();
    println!(
        "Number of times {word:} appears: {}",
        word_search.search(word)
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn example() {}
}
