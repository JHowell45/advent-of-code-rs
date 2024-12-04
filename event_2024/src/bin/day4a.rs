use core::file_reader::get_file_contents;

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

    pub fn word_count(&self, word: &str) -> usize {
        let mut count: usize = 0;
        let len = word.len();
        for (index, c) in self.letters.iter().enumerate() {
            // Left:
            if index > len {
                let local_word: String = self.letters[index - len..index].iter().rev().collect();
                if word == local_word.as_str() {
                    count += 1;
                }
            }
            // Right:
            if index % self.columns < self.columns - len {
                let local_word: String = self.letters[index..index + len].iter().collect();
                if word == local_word.as_str() {
                    count += 1;
                }
            }
            // Top:
            if (index as i32) - (self.columns * len) as i32 > 0 {
                let mut chars: Vec<char> = Vec::with_capacity(len);
                for i in 0..len {
                    chars.insert(i, self.letters[index - (self.columns * i)]);
                }
                let local_word: String = chars.iter().collect();
                if word == local_word.as_str() {
                    count += 1;
                }
            }

            // Diag Top Left:
            if (index as i32) - (self.columns * len) as i32 - len as i32 > 0 {
                let mut chars: Vec<char> = Vec::with_capacity(len);
                for i in 0..len {
                    chars.insert(i, self.letters[index - (self.columns * i) - i]);
                }
                let local_word: String = chars.iter().collect();
                if word == local_word.as_str() {
                    count += 1;
                }
            }

            // Diag Top Right:
            if (index as i32) - (self.columns * len) as i32 + len as i32 > 0 {
                let mut chars: Vec<char> = Vec::with_capacity(len);
                for i in 0..len {
                    chars.insert(i, self.letters[index - (self.columns * i) + i]);
                }
                let local_word: String = chars.iter().collect();
                if word == local_word.as_str() {
                    count += 1;
                }
            }

            // Bottom:
            if index + (self.columns * len) < self.letters.len() {
                let mut chars: Vec<char> = Vec::with_capacity(len);
                for i in 0..len {
                    chars.insert(i, self.letters[index + (self.columns * i)]);
                }
                let local_word: String = chars.iter().collect();
                if word == local_word.as_str() {
                    count += 1;
                }
            }

            // Diag Bottom left:
            if index + (self.columns * len) - len < self.letters.len() {
                let mut chars: Vec<char> = Vec::with_capacity(len);
                for i in 0..len {
                    chars.insert(i, self.letters[index + (self.columns * i) - i]);
                }
                let local_word: String = chars.iter().collect();
                if word == local_word.as_str() {
                    count += 1;
                }
            }

            // Diag Bottom Right:
            if index + (self.columns * len) + len < self.letters.len() {
                let mut chars: Vec<char> = Vec::with_capacity(len);
                for i in 0..len {
                    chars.insert(i, self.letters[index + (self.columns * i) + i]);
                }
                let local_word: String = chars.iter().collect();
                if word == local_word.as_str() {
                    count += 1;
                }
            }
        }
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
        println!("{:?}", search);
        assert_eq!(search.word_count("XMAS"), count);
    }
}