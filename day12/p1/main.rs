use std::env;
use std::error::Error;
use std::fs;

#[derive(Debug)]
enum Reg {
    A,
    B,
    C,
    D,
}

impl Reg {
    fn idx(&self) -> usize {
        match self {
            Reg::A => 0,
            Reg::B => 1,
            Reg::C => 2,
            Reg::D => 3,
        }
    }
}

#[derive(Debug)]
enum Op {
    Register(Reg),
    Number(i32),
}

#[derive(Debug)]
enum Instruction {
    CPY { value: Op, dst: Op },
    INC(Op),
    DEC(Op),
    JNZ { cond: Op, count: Op },
}

struct Parser {
    input_data: Vec<String>,
    curr: usize,
}

impl Parser {
    fn tokenize(filename: &str) -> Result<Self, Box<dyn Error>> {
        let input_data = fs::read_to_string(filename)?
            .lines()
            .map(|line| line.split_whitespace())
            .flatten()
            .map(|token| token.to_string())
            .collect();

        Ok(Parser {
            input_data,
            curr: 0,
        })
    }

    fn parse_next_token_as_operand(&mut self) -> Op {
        let name = self.get_next_token();

        match name {
            "a" => Op::Register(Reg::A),
            "b" => Op::Register(Reg::B),
            "c" => Op::Register(Reg::C),
            "d" => Op::Register(Reg::D),
            num => Op::Number(num.parse().unwrap()),
        }
    }

    fn get_next_token(&mut self) -> &str {
        self.curr += 1;
        self.input_data[self.curr - 1].as_ref()
    }
}

impl Iterator for Parser {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr == self.input_data.len() {
            None
        } else {
            Some(match self.get_next_token() {
                "cpy" => Instruction::CPY {
                    value: self.parse_next_token_as_operand(),
                    dst: self.parse_next_token_as_operand(),
                },
                "inc" => Instruction::INC(self.parse_next_token_as_operand()),
                "dec" => Instruction::DEC(self.parse_next_token_as_operand()),
                "jnz" => Instruction::JNZ {
                    cond: self.parse_next_token_as_operand(),
                    count: self.parse_next_token_as_operand(),
                },
                _ => unreachable!("no other instruction types"),
            })
        }
    }
}

struct Machine {
    registers: [i32; 4],
    pc: usize,
}

impl Machine {
    fn new() -> Self {
        Machine {
            registers: [0; 4],
            pc: 0,
        }
    }

    fn run(&mut self, program: &Vec<Instruction>) {
        while self.pc < program.len() {
            match &program[self.pc] {
                Instruction::INC(op) => match op {
                    Op::Number(_) => {}
                    Op::Register(reg) => self.registers[reg.idx()] += 1,
                },
                Instruction::DEC(op) => match op {
                    Op::Number(_) => {}
                    Op::Register(reg) => self.registers[reg.idx()] -= 1,
                },
                Instruction::CPY { value, dst } => match dst {
                    Op::Number(_) => {}
                    Op::Register(reg) => {
                        self.registers[reg.idx()] = match value {
                            Op::Register(reg) => self.registers[reg.idx()],
                            Op::Number(value) => *value,
                        }
                    }
                },
                Instruction::JNZ { cond, count } => match cond {
                    Op::Register(reg) => {
                        if self.registers[reg.idx()] != 0 {
                            if let Op::Number(count) = count {
                                self.pc = (self.pc as i32 + count - 1) as usize;
                            }
                        };
                    }
                    Op::Number(value) => {
                        if *value != 0 {
                            if let Op::Number(count) = count {
                                self.pc = (self.pc as i32 + count - 1) as usize;
                            }
                        };
                    }
                },
            }
            // println!("{} {:?}", self.pc, self.registers);
            self.pc += 1;
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("--- Day 12: Leonardo's Monorail ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("expecting input file name as cli argument")?;

    let program = Parser::tokenize(&input_file)?.collect::<Vec<Instruction>>();

    let mut machine = Machine::new();
    machine.run(&program);
    println!("{}", machine.registers[Reg::A.idx()]);

    let mut machine = Machine::new();
    machine.registers[Reg::C.idx()] = 1;
    machine.run(&program);
    println!("{}", machine.registers[Reg::A.idx()]);

    Ok(())
}
