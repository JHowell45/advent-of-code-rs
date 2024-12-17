use itertools::Itertools;

pub enum Registers {
    A, B, C
}

pub struct Computer {
    instruction_p: usize
}

impl Computer {
    pub fn new() -> Self {
        Self {
            instruction_p: 0
        }
    }

    pub fn run(&self, instructions: Vec<i32>) {
        for (op, operand) in instructions.chunks(2).into_iter().map(|chunk| (chunk[0], chunk[1])) {
            println!("{op} => {operand:}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
}