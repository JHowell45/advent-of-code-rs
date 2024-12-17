use core::file_reader::get_file_contents;

use event_2024::shared::day17::{Computer, ProgramDuplicator};

fn main() {
    let (_, program) = Computer::from_string(get_file_contents(2024, 17).as_str());
    let mut dup = ProgramDuplicator::new(program);
    println!("Smallest A {}", dup.smallest_a());
}

#[cfg(test)]
mod tests {
    use super::*;
    use event_2024::shared::day17::ProgramDuplicator;
    use rstest::rstest;

    #[rstest]
    fn example() {
        let program = vec![0, 3, 5, 4, 3, 0];
        let mut duplicator = ProgramDuplicator::new(program);
        assert_eq!(duplicator.smallest_a(), 117440);
    }
}
