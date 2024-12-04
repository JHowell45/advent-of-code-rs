use core::file_reader::get_file_contents;
use std::ops::Index;

fn main() {
    let search = WordSearch::from_string(get_file_contents(2024, 4));
    println!("{:?}", search);
}

#[derive(Debug)]
struct WordSearch {
    letters: Vec<char>,
    columns: usize
}

impl WordSearch {
    pub fn from_string(text: String) -> Self {
        Self {
            columns: text.find("\n").unwrap(),
            letters: text.chars().filter(|c| *c != '\n').collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn example() {}
}