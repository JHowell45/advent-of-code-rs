use std::u64;

use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    ADV,
    BXL,
    BST,
    JNZ,
    BXC,
    OUT,
    BDV,
    CDV,
}

impl From<i8> for OpCode {
    fn from(value: i8) -> Self {
        match value {
            0 => Self::ADV,
            1 => Self::BXL,
            2 => Self::BST,
            3 => Self::JNZ,
            4 => Self::BXC,
            5 => Self::OUT,
            6 => Self::BDV,
            7 => Self::CDV,
            _ => panic!("Invalid op code value: {value}!!"),
        }
    }
}

#[derive(Debug)]
pub struct Computer {
    pub registers: [u64; 3],
    pub output: Vec<i8>,
    instruction_p: usize,
    jumped: bool,
}

impl Computer {
    pub fn new() -> Self {
        Self::define_registers(0, 0, 0)
    }

    pub fn define_registers(a: u64, b: u64, c: u64) -> Self {
        Self {
            registers: [a, b, c],
            output: Vec::new(),
            instruction_p: 0,
            jumped: false,
        }
    }

    pub fn from_string(information: &str) -> (Self, Vec<i8>) {
        let pattern = Regex::new(r"Register A: (?<a>\d+)\nRegister B: (?<b>\d+)\nRegister C: (?<c>\d+)\n\nProgram: (?<program>.+)").unwrap();
        let caps = pattern.captures(&information).unwrap();
        let instance = Self::define_registers(
            caps["a"].parse::<u64>().unwrap(),
            caps["b"].parse::<u64>().unwrap(),
            caps["c"].parse::<u64>().unwrap(),
        );
        return (
            instance,
            caps["program"]
                .split(",")
                .map(|v| v.parse::<i8>().unwrap())
                .collect(),
        );
    }

    pub fn smallest_a(&mut self, instructions: Vec<i8>) -> u64 {
        let mut starting_a: u64 = u64::MAX;
        // self.instruction_p = instructions.len();
        while self.output != instructions {
            self.reset(starting_a, 0, 0);
            // println!("START: {self:?}");
            while self.instruction_p < instructions.len() - 1 {
                let op: OpCode = OpCode::from(instructions[self.instruction_p]);
                let operand: i8 = instructions[self.instruction_p + 1];
                self.run_instruction(op, operand);
                if self.output.len() > 1 {
                    if self.output != instructions[0..self.output.len()] {
                        break;
                    } else {

                        println!("{self:?}");
                    }
                }
                // println!(
                //     "{} : {op:?} -> {operand} || {:?} | {:?}",
                //     self.instruction_p / 2,
                //     self.registers,
                //     self.output
                // );
                match self.jumped {
                    true => self.jumped = false,
                    false => self.instruction_p += 2,
                }
            }
            // println!("{self:?}");
            starting_a -= 1;
        }
        // while self.instruction_p > 0 {
        //     match self.jumped {
        //         true => self.jumped = false,
        //         false => self.instruction_p -= 2,
        //     }
        //     let op: OpCode = OpCode::from(instructions[self.instruction_p]);
        //     let operand: i8 = instructions[self.instruction_p + 1];
        //     self.run_instruction(op, operand);
        //     println!("{} : {op:?} -> {operand} || {:?} | {:?}", self.instruction_p / 2, self.registers, self.output);
        // }
        return self.registers[0];
    }

    pub fn run(&mut self, instructions: Vec<i8>) {
        while self.instruction_p < instructions.len() - 1 {
            let op: OpCode = OpCode::from(instructions[self.instruction_p]);
            let operand: i8 = instructions[self.instruction_p + 1];
            self.run_instruction(op, operand);
            println!(
                "{} : {op:?} -> {operand} || {:?} | {:?}",
                self.instruction_p / 2,
                self.registers,
                self.output
            );
            match self.jumped {
                true => self.jumped = false,
                false => self.instruction_p += 2,
            }
        }
    }

    pub fn run_instruction(&mut self, op: OpCode, operand: i8) {
        match op {
            OpCode::ADV => self.adv(operand),
            OpCode::BXL => self.bxl(operand),
            OpCode::BST => self.bst(operand),
            OpCode::JNZ => self.jnz(operand),
            OpCode::BXC => self.bxc(operand),
            OpCode::OUT => self.out(operand),
            OpCode::BDV => self.bdv(operand),
            OpCode::CDV => self.cdv(operand),
        }
    }

    pub fn output(&self) -> String {
        self.output
            .iter()
            .enumerate()
            .map(|(i, v)| match i {
                0 => format!("{v}"),
                _ => format!(",{v}"),
            })
            .collect()
    }

    fn combo_operand(&self, operand: i8) -> u64 {
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.registers[0],
            5 => self.registers[1],
            6 => self.registers[2],
            _ => panic!("Invalid operand number! '{operand:}'"),
        }
    }

    fn adv(&mut self, operand: i8) {
        self.registers[0] /= 2_i32.pow(self.combo_operand(operand) as u32) as u64
    }

    fn bxl(&mut self, operand: i8) {
        self.registers[1] = self.registers[1] ^ operand as u64
    }

    fn bst(&mut self, operand: i8) {
        self.registers[1] = self.combo_operand(operand) % 8
    }

    fn jnz(&mut self, operand: i8) {
        match self.registers[0] {
            0 => {}
            _ => {
                self.instruction_p = operand as usize;
                self.jumped = true;
            }
        }
    }

    fn bxc(&mut self, _operand: i8) {
        self.registers[1] ^= self.registers[2]
    }

    fn out(&mut self, operand: i8) {
        self.output.push((self.combo_operand(operand) % 8) as i8)
    }

    fn bdv(&mut self, operand: i8) {
        self.registers[1] = self.registers[0] / 2_i32.pow(self.combo_operand(operand) as u32) as u64
    }

    fn cdv(&mut self, operand: i8) {
        self.registers[2] = self.registers[0] / 2_i32.pow(self.combo_operand(operand) as u32) as u64
    }

    fn reset(&mut self, a: u64, b: u64, c: u64) {
        self.output = Vec::new();
        self.registers[0] = a;
        self.registers[1] = b;
        self.registers[2] = c;
        self.jumped = false;
        self.instruction_p = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case([0, 0, 9], vec![2, 6], [0, 1, 9], vec![])]
    #[case([10, 0, 0], vec![5,0,5,1,5,4], [10, 0, 0], vec![0, 1, 2])]
    #[case([2024, 0, 0], vec![0,1,5,4,3,0], [0, 0, 0], vec![4,2,5,6,7,7,7,7,3,1,0])]
    #[case([0, 29, 0], vec![1,7], [0, 26, 0], vec![])]
    #[case([0, 2024, 43690], vec![4, 0], [0, 44354, 43690], vec![])]
    fn test_instructions(
        #[case] registers: [u64; 3],
        #[case] program: Vec<i8>,
        #[case] expected_registers: [u64; 3],
        #[case] expected_output: Vec<i8>,
    ) {
        let mut computer = Computer::define_registers(registers[0], registers[1], registers[2]);
        computer.run(program);
        assert_eq!(computer.registers, expected_registers);
        assert_eq!(computer.output, expected_output);
    }
}
