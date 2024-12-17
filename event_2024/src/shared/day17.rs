use itertools::Itertools;

pub struct Computer {
    registers: [usize; 3],
    instruction_p: usize,
}

impl Computer {
    pub fn new() -> Self {
        Self {
            registers: [0; 3],
            instruction_p: 0,
        }
    }

    pub fn run(&mut self, instructions: Vec<i32>) {
        while self.instruction_p < instructions.len() - 1 {
            let op = instructions[self.instruction_p];
            let operand = instructions[self.instruction_p + 1];

            println!("{:}: {op:} => {operand:}", self.instruction_p);
            self.instruction_p += 2;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
}
