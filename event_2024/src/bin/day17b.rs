use core::file_reader::get_file_contents;

use event_2024::shared::day17::Computer;

fn main() {
    let (mut computer, program) = Computer::from_string(get_file_contents(2024, 17).as_str());
    // computer.run(program);
    computer.smallest_a(program);
    // let mut dup = ProgramDuplicator::new(program);
    // println!("Smallest A {:?}", dup.smallest_a());
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn example() {
        let program = vec![0, 3, 5, 4, 3, 0];
        // let mut duplicator = ProgramDuplicator::new(program);
        // assert_eq!(duplicator.smallest_a(), Some(117440));
        let mut comp = Computer::define_registers(2024, 0, 0);
        // comp.run(program.clone());
        // println!("\n");
        assert_eq!(comp.smallest_a(program), 117440);
    }
}
