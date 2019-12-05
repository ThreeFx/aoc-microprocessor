use std::convert::TryFrom;
use std::io::BufRead;
use std::str::FromStr;

use num_enum::TryFromPrimitive;

#[derive(Debug, PartialEq, TryFromPrimitive)]
#[repr(i32)]
enum ParameterMode {
    Memory = 0,
    Immediate = 1,
}

#[derive(Debug, PartialEq, TryFromPrimitive)]
#[repr(i32)]
enum InstructionType {
    Add = 1,
    Multiply = 2,

    Read = 3,
    Print = 4,

    JumpNZ = 5,
    JumpZ = 6,
    IsLessThan = 7,
    IsEqual = 8,

    Halt = 99,
}

#[derive(Debug)]
struct Processor {
    memory: Vec<i32>,
    ip: usize,
}

#[derive(Debug)]
struct Instruction {
    i_type: InstructionType,
    p1_mode: ParameterMode,
    p2_mode: ParameterMode,
    p3_mode: ParameterMode,
}

fn main() {
    let stdin = std::io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();

    let program = line
        .split(',')
        .map(FromStr::from_str)
        .map(Result::unwrap)
        .collect();

    let mut processor = Processor::initialize(program);
    processor.run();
    println!("");
    //println!("{:?}", processor.memory);
}

impl Processor {
    fn initialize(program: Vec<i32>) -> Processor {
        return Processor {
            memory: program,
            ip: 0,
        }
    }

    fn run(&mut self) {
        while !self.step() { }
    }

    fn step(&mut self) -> bool {
        let i_type = self.current_instruction().i_type;

        let _ = match i_type {
            InstructionType::Add => self.binary_operation(&|(p1, p2)| p1 + p2),
            InstructionType::Multiply => self.binary_operation(&|(p1, p2)| p1 * p2),

            InstructionType::Read => self.read(),
            InstructionType::Print => self.print(),

            InstructionType::JumpNZ => self.conditional_jump(&|p1| p1 != 0),
            InstructionType::JumpZ => self.conditional_jump(&|p1| p1 == 0),

            InstructionType::IsLessThan => self.binary_operation(&|(p1, p2)| if p1 < p2 { 1 } else { 0 }),
            InstructionType::IsEqual => self.binary_operation(&|(p1, p2)| if p1 == p2 { 1 } else { 0 }),

            InstructionType::Halt => return true,
        };

        return false;
    }

    fn binary_operation(&mut self, op: &dyn Fn((i32, i32)) -> i32) -> Result<(), String> {
        let instruction = self.current_instruction();

        let p1 = self.get_parameter_with_mode(1, instruction.p1_mode);
        let p2 = self.get_parameter_with_mode(2, instruction.p2_mode);

        if instruction.p3_mode == ParameterMode::Immediate {
            return Err("got immediate parameter mode for store address".to_string())
        }
        let p3 = self.get_parameter(3);

        self.memory[p3 as usize] = op((p1, p2));
        self.ip += 4;
        return Ok(());
    }

    fn read(&mut self) -> Result<(), String> {
        let instruction = self.current_instruction();

        if instruction.p1_mode == ParameterMode::Immediate {
            return Err("got immediate parameter mode for store address".to_string())
        }
        let p1 = self.get_parameter(1);

        let stdin = std::io::stdin();
        let line = stdin.lock().lines().next().unwrap().unwrap();
        let input = FromStr::from_str(&line).unwrap();

        self.memory[p1 as usize] = input;
        self.ip += 2;
        return Ok(())
    }

    fn print(&mut self) -> Result<(), String> {
        let instruction = self.current_instruction();
        let p1 = self.get_parameter_with_mode(1, instruction.p1_mode);

        print!("{}", p1);

        self.ip += 2;
        return Ok(())
    }

    fn conditional_jump(&mut self, condition: &dyn Fn(i32) -> bool) -> Result<(), String> {
        let instruction = self.current_instruction();
        let p1 = self.get_parameter_with_mode(1, instruction.p1_mode);
        let p2 = self.get_parameter_with_mode(2, instruction.p2_mode);

        if condition(p1) {
            self.ip = p2 as usize;
        } else {
            self.ip += 3;
        }

        return Ok(())
    }

    fn current_instruction(&self) -> Instruction {
        let instruction = self.memory[self.ip];

        let i_type = InstructionType::try_from(instruction % 100)
            .expect("invalid opcode");
        let p1_mode = Processor::parse_mode((instruction / 100) % 10);
        let p2_mode = Processor::parse_mode((instruction / 1000) % 10);
        let p3_mode = Processor::parse_mode((instruction / 10000) % 10);

        return Instruction {
            i_type: i_type,
            p1_mode: p1_mode,
            p2_mode: p2_mode,
            p3_mode: p3_mode,
        }
    }

    fn get_parameter(&self, i: usize) -> i32 {
        return self.memory[self.ip + i];
    }

    fn get_parameter_with_mode(&self, i: usize, mode: ParameterMode) -> i32 {
        return match mode {
            ParameterMode::Memory => self.memory[self.memory[self.ip + i] as usize],
            ParameterMode::Immediate => self.memory[self.ip + i],
        }
    }

    fn parse_mode(mode: i32) -> ParameterMode {
        return ParameterMode::try_from(mode)
            .expect("invalid parameter mode");
    }
}
