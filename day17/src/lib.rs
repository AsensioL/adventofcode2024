use regex::Regex;
use thiserror::Error;
use itertools::Itertools;

use u64 as Register;

#[derive(Error, Debug)]
pub enum ComputerError {
    #[error("Cannot access instruction {position:?}, Program Length {program_length:?})")]
    CannotAccessInstruction {
        position: usize,
        program_length: usize,
    },
    #[error("Cannot access parameter at {position:?}, Program Length {program_length:?})")]
    CannotAccessParam {
        position: usize,
        program_length: usize,
    },
    #[error("Undefined error")]
    Undefined,
}

struct Computer {
    a: Register,
    b: Register,
    c: Register,
    program: Vec<Register>,
    pc: usize,
    increase_pc: bool,
    output: Vec<Register>,
    initial_state: (Register, Register, Register)
}

type ComputerResult = Result<(),ComputerError>;
//use Result<(),ComputerError> as ComputerResult;

impl Computer {
    fn new(a: Register, b: Register, c: Register, program: Vec<Register>) -> Self {
        Self { a, b, c, program, pc: 0, increase_pc: true, output: Vec::new(), initial_state: (a, b, c)}
    }

    fn restart(&mut self) {
        (self.a, self.b, self.c) = self.initial_state;
        self.pc = 0;
        self.increase_pc = true;
        self.output.clear()
    }

    fn finished(&self) -> bool {
        self.pc == self.program.len() //.try_into().expect("Failed to convert program length to Register type")
    }

    fn run(&mut self) -> ComputerResult {
        while !self.finished() {
            self.step()?;
        }
        Ok(())
    }

    fn get_instruction(&self) -> Result<&Register, ComputerError> {
        // Error if program cannot get
        self.program.get(self.pc)
            .ok_or(ComputerError::CannotAccessInstruction { position: self.pc, program_length: self.program.len() })
    }

    fn get_param(&self) -> Result<&Register, ComputerError> {
        // Error if program cannot get
        self.program.get(self.pc + 1)
            .ok_or(ComputerError::CannotAccessParam { position: self.pc + 1, program_length: self.program.len() })
    }

    fn combo(&self) -> Result<Register, ComputerError> {
        match self.get_param()? {
            0 => Ok(0),
            1 => Ok(1),
            2 => Ok(2),
            3 => Ok(3),
            4 => Ok(self.a),
            5 => Ok(self.b),
            6 => Ok(self.c),
            _ => unimplemented!()
        }
    }

    fn literal(&self) -> Result<Register, ComputerError> {
        match self.get_param()? {
            0 => Ok(0),
            1 => Ok(1),
            2 => Ok(2),
            3 => Ok(3),
            4 => Ok(4),
            5 => Ok(5),
            6 => Ok(6),
            7 => Ok(7),
            _ => unimplemented!()
        }
    }

    fn step(&mut self) -> ComputerResult {
        // Handle instruction
        match self.get_instruction()? {
            0 => self.op_adv()?,
            1 => self.op_bxl()?,
            2 => self.op_bst()?,
            3 => self.op_jnz()?,
            4 => self.op_bxc()?,
            5 => self.op_out()?,
            6 => self.op_bdv()?,
            7 => self.op_cdv()?,
            _ => unimplemented!()
        }

        // Handle increasing PC
        if self.increase_pc {
            self.pc += 2;
        }
        self.increase_pc = true;

        Ok( () )
    }

    fn op_adv(&mut self) -> ComputerResult { self.a >>= self.combo()?; Ok(()) }
    fn op_bxl(&mut self) -> ComputerResult { self.b ^= self.literal()?; Ok(()) }
    fn op_bst(&mut self) -> ComputerResult { self.b = self.combo()? % 8; Ok(()) }
    fn op_jnz(&mut self) -> ComputerResult { if self.a == 0 { Ok(()) } else { self.pc = self.literal()?.try_into().unwrap(); self.increase_pc = false; Ok(() )} }
    fn op_bxc(&mut self) -> ComputerResult { self.b ^= self.c; Ok(()) }
    fn op_out(&mut self) -> ComputerResult { self.output.push(self.combo()? % 8); Ok(()) }
    fn op_bdv(&mut self) -> ComputerResult { self.b = self.a >> self.combo()?; Ok(()) }
    fn op_cdv(&mut self) -> ComputerResult { self.c = self.a >> self.combo()?; Ok(()) }
}

pub fn part1(input: &str) -> String
{
    let mut cpu = {
        let mut a = 0;
        let mut b = 0;
        let mut c = 0;
        let mut program = Vec::new();

        let re = Regex::new(r"(?<key>Register [ABC]|Program): (?<data>[\d,]+)").unwrap();

        for (key, data) in re.captures_iter(input).map(|caps| (caps.name("key").unwrap().as_str(), caps.name("data").unwrap().as_str()) ) {
            match key {
                "Register A" => a = data.parse::<Register>().unwrap_or_else(|_| panic!("Error parsing A: `{data}`") ),
                "Register B" => b = data.parse::<Register>().unwrap_or_else(|_| panic!("Error parsing B: `{data}`") ),
                "Register C" => c = data.parse::<Register>().unwrap_or_else(|_| panic!("Error parsing C: `{data}`") ),
                "Program"    => program = data.split(',').map(|n_txt| n_txt.parse::<Register>().unwrap() ).collect::<Vec<_>>(),
                _  => panic!("Unknown input {key}"),
            };
        }

        Computer::new(a, b, c, program)
    };

    // Run and print if there are any errors
    let result = cpu.run();

    // Panic if the CPU ended in a bad state
    if let Err(error) = result {
        panic!("Error: {error}");
    }

    cpu.output.iter().map(Register::to_string).join(",")
}

pub fn part2(input: &str) -> Register
{
    let mut cpu = {
        let mut a = 0;
        let mut b = 0;
        let mut c = 0;
        let mut program = Vec::new();

        let re = Regex::new(r"(?<key>Register [ABC]|Program): (?<data>[\d,]+)").unwrap();

        for (key, data) in re.captures_iter(input).map(|caps| (caps.name("key").unwrap().as_str(), caps.name("data").unwrap().as_str()) ) {
            match key {
                "Register A" => a = data.parse::<Register>().unwrap_or_else(|_| panic!("Error parsing A: `{data}`") ),
                "Register B" => b = data.parse::<Register>().unwrap_or_else(|_| panic!("Error parsing B: `{data}`") ),
                "Register C" => c = data.parse::<Register>().unwrap_or_else(|_| panic!("Error parsing C: `{data}`") ),
                "Program"    => program = data.split(',').map(|n_txt| n_txt.parse::<Register>().unwrap() ).collect::<Vec<_>>(),
                _  => panic!("Unknown input {key}"),
            };
        }

        Computer::new(a, b, c, program)
    };

    // After looking at the MY input, here's the translation from decimal to 3-bit assembly:
    //  0: bst 4; // store in b the modulo 8 of combo(4), which is (reg_a)
    //  2: bxl 2; // store in b the result of (reg_b) XOR literal(2)
    //  4: cdv 5; // store in c the result of (reg_a) >> combo(5), which is (reg_b)
    //  6: bxc 5; // store in b the result of (reg_b) XOR (reg_c), ignore the 5
    // 10: bxl 3; // store in b the result of (reg_b) XOR literal(3)
    // 12: out 5; // print the modulo 8 of combo(5), which is (reg_b)
    // 14: adv 3; // store in a the result of (reg_a) >> combo(3), which is 3
    // 16: jnz 0; // jump to instruction 0, otherwise continue

    // In computer notation (not that ^ is XOR)
    // while A > 0:
    //   B = A % 8
    //   B = B ^ 2
    //   C = A >> B
    //   B = B ^ C
    //   B = B ^ 3
    //   out( B % 8 )
    //   A = A >> 3

    // In mathematical notation, after simplifications (note that ^ is power and // is rounded-down division)
    //   Our program has 8 opcodes each with an additional value, for a total of 16 3-bit values
    //   Consider A(n) = Sum[m=0, 16][a_m * 8 ^ (m - n)], where a_m are in range [0, 7]
    //     out_n = a_n XOR [ A(n) // 2^( a_n XOR 2 ) ] XOR 1

    // Note that the output only depends on the initial value of (reg_a) on every iteration

    // Also if we expand the equations for n = 13, 14, 15:
    //   out_13 = a_13 [ (a_13 + a_14*8 + a_15*8^2) // 2^( a_13 XOR 2 ) ] XOR 1
    //   out_14 = a_14 [ (a_14 + a_15*8)            // 2^( a_14 XOR 2 ) ] XOR 1
    //   out_15 = a_15 [  a_15                      // 2^( a_15 XOR 2 ) ] XOR 1

    // Note that out_n are known and that the last equation only has 1 unknown (a_15)
    // Using this, we can test solutions 3 bits at a time (there might be multiple correct solutions)
    // This should allow solving the problem with complexity of ~ 8 * number_of_values_in_program = 8 * 16

    let mut a_as_3bit_numbers = vec![];
    let mut start_at = 0; // Used for backtracking from dead-end solutions

    while cpu.output != cpu.program {
        // Try each value
        let mut found = false;

        for val in start_at..=7 {
            // Calculate the full value of register A for the current test
            let mut copy_of_a = a_as_3bit_numbers.clone();
            copy_of_a.push(val);
            let subset_of_a: Register = copy_of_a.iter()
                .rev()
                .enumerate()
                .map(|(idx, val)| val * 8_u64.pow(idx as u32) )
                .sum();

            // Assign A and run the program
            cpu.restart();
            cpu.a = subset_of_a;
            let _ = cpu.run();

            // If the output does not have the expected length, this is not a valid output, so keep trying
            if cpu.output.len() != copy_of_a.len() {
                continue;
            }

            // Compare the endings, if they match, save the value and move on to the next 3-bit number
            if cpu.output.iter().rev().zip( cpu.program.iter().rev() ).all(|(output_v, program_v)| output_v == program_v ) {
                a_as_3bit_numbers.push(val);
                found = true;
                start_at = 0;
                break;
            }
        }

        // If we reached 7 and a solution was not found, the previous solution might have been bad, so backtrack
        // and start testing again from one number higher
        if !found {
            // The previous number must have led to a dead-end, backtrack
            start_at = a_as_3bit_numbers.pop().expect("Backtracked the entire vector") + 1;
        }
    }

    // Return the full A register
    a_as_3bit_numbers.iter()
        .rev()
        .enumerate()
        .map(|(idx, val)| val * 8_u64.pow(idx as u32) )
        .sum::<Register>()
}


// ------------ Tests ------------
#[cfg(test)]
mod tests {
    use std::vec;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_example_1() {
        let a = 99999;
        let b = 99999;
        let c = 9;
        let program = vec![2,6];
        let mut cpu = Computer::new(a, b, c, program);

        let final_state = cpu.run();

        assert!(final_state.is_ok());
        assert_eq!(cpu.b,  1);
    }

    #[test]
    fn test_example_2() {
        let a = 10;
        let b = 99999;
        let c = 99999;
        let program = vec![5,0,5,1,5,4];
        let mut cpu = Computer::new(a, b, c, program);

        let final_state = cpu.run();

        assert!(final_state.is_ok());
        assert_eq!(cpu.output,  vec![0,1,2]);
    }

    #[test]
    fn test_example_3() {
        let a = 2024;
        let b = 99999;
        let c = 99999;
        let program = vec![0,1,5,4,3,0];
        let mut cpu = Computer::new(a, b, c, program);

        let final_state = cpu.run();

        assert!(final_state.is_ok());
        assert_eq!(cpu.output,  vec![4,2,5,6,7,7,7,7,3,1,0]);
        assert_eq!(cpu.a,  0);
    }

    #[test]
    fn test_example_4() {
        let a = 99999;
        let b = 29;
        let c = 99999;
        let program = vec![1,7];
        let mut cpu = Computer::new(a, b, c, program);

        let final_state = cpu.run();

        assert!(final_state.is_ok());
        assert_eq!(cpu.b,  26);
    }

    #[test]
    fn test_example_5() {
        let a = 99999;
        let b = 2024;
        let c = 43690;
        let program = vec![4,0];
        let mut cpu = Computer::new(a, b, c, program);

        let final_state = cpu.run();

        assert!(final_state.is_ok());
        assert_eq!(cpu.b,  44354);
    }

    #[test]
    fn test_example_bdv() {
        let a = 1024;
        let b = 3;
        let c = 99999;
        let program = vec![6,5];
        let mut cpu = Computer::new(a, b, c, program);

        let final_state = cpu.run();

        assert!(final_state.is_ok());
        assert_eq!(cpu.b,  128);
    }

    #[test]
    fn test_example_cdv() {
        let a = 1024;
        let b = 3;
        let c = 99999;
        let program = vec![7,5];
        let mut cpu = Computer::new(a, b, c, program);

        let final_state = cpu.run();

        assert!(final_state.is_ok());
        assert_eq!(cpu.c,  128);
    }
}