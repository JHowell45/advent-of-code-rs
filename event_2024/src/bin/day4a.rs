use core::file_reader::get_file_contents;
use std::process::exit;

fn main() {
    let search = WordSearch::from_string(get_file_contents(2024, 4));
    println!("How many times does 'XMAS' appear: {}", search.word_count("XMAS"));
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

    pub fn word_count(&self, word: &str) -> usize {
        let mut count: usize = 0;
        let len = word.len();
        let _first_char: char = word.chars().next().unwrap();

        let column_check = self.columns * (len - 1);

        for (index, _c) in self.letters.iter().enumerate() {
            // Left:
            if index >= len - 1 {
                let idx = index - (len - 1);
                let local_word: String = self.letters[idx..=index].iter().rev().collect();
                if word == local_word.as_str() {
                    count += 1;
                }
            }

            // Right:
            if index % self.columns < self.columns - len + 1 {
                let local_word: String = self.letters[index..index + len].iter().collect();
                if word == local_word.as_str() {
                    count += 1;
                }
            }

            // Top:
            if (index as i32) - column_check as i32 > 0 {
                let mut chars: Vec<char> = Vec::with_capacity(len);
                for i in 0..len {
                    chars.insert(i, self.letters[index - (self.columns * i)]);
                }
                let local_word: String = chars.iter().collect();
                println!("{local_word:}");
                if word == local_word.as_str() {
                    count += 1;
                }
            }

            // // Diag Top Left:
            // if (index as i32) - column_check as i32 - len as i32 + 1 > 0 {
            //     let mut chars: Vec<char> = Vec::with_capacity(len);
            //     for i in 0..len {
            //         chars.insert(i, self.letters[index - (self.columns * i) - i]);
            //     }
            //     let local_word: String = chars.iter().collect();
            //     if word == local_word.as_str() {
            //         count += 1;
            //     }
            // }

            // // Diag Top Right:
            // if (index as i32) - column_check as i32 + len as i32 > 0 {
            //     let mut chars: Vec<char> = Vec::with_capacity(len);
            //     for i in 0..len {
            //         chars.insert(i, self.letters[index - (self.columns * i) + i]);
            //     }
            //     let local_word: String = chars.iter().collect();
            //     if word == local_word.as_str() {
            //         count += 1;
            //     }
            // }

            // // Bottom:
            // if index + column_check < self.letters.len() {
            //     let mut chars: Vec<char> = Vec::with_capacity(len);
            //     for i in 0..len {
            //         chars.insert(i, self.letters[index + (self.columns * i)]);
            //     }
            //     let local_word: String = chars.iter().collect();
            //     if word == local_word.as_str() {
            //         count += 1;
            //     }
            // }

            // // Diag Bottom left:
            // if index + column_check - len < self.letters.len() {
            //     let mut chars: Vec<char> = Vec::with_capacity(len);
            //     for i in 0..len {
            //         let idx = index + (self.columns * i) - i;
            //         chars.insert(i, self.letters[idx]);
            //     }
            //     let local_word: String = chars.iter().collect();
            //     if word == local_word.as_str() {
            //         count += 1;
            //     }
            // }

            // // Diag Bottom Right:
            // if index + column_check + len < self.letters.len() {
            //     let mut chars: Vec<char> = Vec::with_capacity(len);
            //     for i in 0..len {
            //         chars.insert(i, self.letters[index + (self.columns * i) + i]);
            //     }
            //     let local_word: String = chars.iter().collect();
            //     if word == local_word.as_str() {
            //         count += 1;
            //     }
            // }
        }
        println!("{:?}", self);
        return count;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX", 18)]
    fn example(#[case] text: String, #[case] count: usize) {
        let search = WordSearch::from_string(text);
        assert_eq!(search.word_count("XMAS"), count);
    }
}