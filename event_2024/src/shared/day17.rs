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

    pub fn run(&self, instructions: Vec<i32>) {
        for (p, (op, operand)) in instructions
            .chunks(2)
            .into_iter()
            .map(|chunk| (chunk[0], chunk[1]))
            .enumerate()
        {
            println!("{p:}: {op:} => {operand:}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
}
