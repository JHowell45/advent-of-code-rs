use core::file_reader::get_file_contents;

use event_2024::shared::day17::Computer;

fn main() {
    let (mut computer, program) = Computer::from_string(get_file_contents(2024, 17).as_str());
    println!("Smallest A {}", computer.copy_program_smallest_a(program));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn example() {
        let program = vec![0, 3, 5, 4, 3, 0];
        let mut computer = Computer::define_registers(2024, 0, 0);
        assert_eq!(computer.copy_program_smallest_a(program), 117440);
    }
}
