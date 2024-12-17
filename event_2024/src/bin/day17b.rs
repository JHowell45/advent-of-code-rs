use core::file_reader::get_file_contents;

use event_2024::shared::day17::Computer;

fn main() {
    let (mut computer, program) = Computer::from_string(get_file_contents(2024, 17).as_str());
    println!("{computer:?}");
    computer.run(program);
    println!("{computer:?}");
    println!("Smallest A {}", program_copy_with_smallest_a_register());
}

fn program_copy_with_smallest_a_register() -> i32 {
    let (_, program) = Computer::from_string(get_file_contents(2024, 17).as_str());
    let mut a: i32 = 0;
    loop {
        let mut computer = Computer::define_registers(a, 0, 0);
        println!("{computer:?}");
        computer.run(program.clone());
        println!("{computer:?}");
        if computer.output == program {
            break;
        }
        a += 1;
    }
    return a;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn example() {
        let mut a: i32 = 0;
        loop {
            let program = vec![0,3,5,4,3,0];
            let mut computer = Computer::define_registers(a, 0, 0);
            computer.run(program.clone());
            if computer.output == program {
                break;
            }
            a += 1;
        }
        assert_eq!(a, 117440)
    }
}
