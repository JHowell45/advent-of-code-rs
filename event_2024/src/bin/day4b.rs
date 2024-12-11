use core::file_reader::get_file_contents;

use event_2024::shared::day4::WordSearch;

fn main() {
    let word: &str = "MAS";
    let word_search = WordSearch::from_string(get_file_contents(2024, 4).as_str());
    word_search.display_search();
    println!(
        "Number of times X-{word:} appears: {}",
        word_search.x_search(word)
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn example() {}
}
