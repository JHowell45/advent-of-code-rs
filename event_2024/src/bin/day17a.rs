use aoc_core::file_reader::get_file_contents;

use event_2024::shared::day17::Computer;

fn main() {
    let (mut computer, program) = Computer::from_string(get_file_contents(2024, 17).as_str());
    println!("{computer:?}");
    computer.run(program);
    println!("{computer:?}");
    println!("{:?}", computer.output());
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn example() {
        let mut com = Computer::define_registers(729, 0, 0);
        com.run(vec![0, 1, 5, 4, 3, 0]);
        assert_eq!(com.output, vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0])
    }
}
