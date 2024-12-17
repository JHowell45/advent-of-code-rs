use event_2024::shared::day17::Computer;

fn main() {
    let mut computer = Computer::define_registers(64196994, 0, 0);
    println!("{computer:?}");
    computer.run(vec![2, 4, 1, 1, 7, 5, 1, 5, 4, 0, 0, 3, 5, 5, 3, 0]);
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
