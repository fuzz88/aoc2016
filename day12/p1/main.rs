use std::env;
use std::error::Error;
use std::fs;

#[derive(Debug)]
enum Op {
    A,
    B,
    C,
    D,
    N(i32),
}

#[derive(Debug)]
enum Instruction {
    CPY { value: Op, dst: Op },
    INC(Op),
    DEC(Op),
    JNZ { cond: Op, count: i32 },
}

struct Parser {
    input_data: Vec<String>,
    curr: usize,
}

impl Parser {
    fn new(filename: &str) -> Result<Self, Box<dyn Error>> {
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
}

fn parse_operand(name: &str) -> Op {
    match name {
        "a" => Op::A,
        "b" => Op::B,
        "c" => Op::C,
        "d" => Op::D,
        num => Op::N(num.parse().unwrap()),
    }
}

impl Iterator for Parser {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        self.curr += 1;
        if self.curr < self.input_data.len() {
            Some(match self.input_data[self.curr - 1].as_ref() {
                "cpy" => {
                    self.curr += 2;
                    Instruction::CPY {
                        value: parse_operand(self.input_data[self.curr - 2].as_ref()),
                        dst: parse_operand(self.input_data[self.curr - 1].as_ref()),
                    }
                }
                "inc" => {
                    self.curr += 1;
                    Instruction::INC(parse_operand(self.input_data[self.curr - 1].as_ref()))
                }
                "dec" => {
                    self.curr += 1;
                    Instruction::DEC(parse_operand(self.input_data[self.curr - 1].as_ref()))
                }
                "jnz" => {
                    self.curr += 2;
                    Instruction::JNZ {
                        cond: parse_operand(self.input_data[self.curr - 2].as_ref()),
                        count: self.input_data[self.curr - 1].parse().unwrap(),
                    }
                }
                _ => unreachable!("no other instruction types"),
            })
        } else {
            None
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
        loop {
            if self.pc >= program.len() {
                break;
            } else {
                match &program[self.pc] {
                    Instruction::INC(op) => match op {
                        Op::A => self.registers[0] += 1,
                        Op::B => self.registers[1] += 1,
                        Op::C => self.registers[2] += 1,
                        Op::D => self.registers[3] += 1,
                        Op::N(_) => {}
                    },
                    Instruction::DEC(op) => match op {
                        Op::A => self.registers[0] -= 1,
                        Op::B => self.registers[1] -= 1,
                        Op::C => self.registers[2] -= 1,
                        Op::D => self.registers[3] -= 1,
                        Op::N(_) => {}
                    },
                    Instruction::CPY { value, dst } => match dst {
                        Op::A => {
                            self.registers[0] = match value {
                                Op::B => self.registers[1],
                                Op::C => self.registers[2],
                                Op::D => self.registers[3],
                                Op::N(v) => *v,
                                _ => unreachable!("cant copy register to itself"),
                            }
                        }
                        Op::B => {
                            self.registers[1] = match value {
                                Op::A => self.registers[0],
                                Op::C => self.registers[2],
                                Op::D => self.registers[3],
                                Op::N(v) => *v,
                                _ => unreachable!("cant copy register to itself"),
                            }
                        }
                        Op::C => {
                            self.registers[2] = match value {
                                Op::B => self.registers[1],
                                Op::A => self.registers[0],
                                Op::D => self.registers[3],
                                Op::N(v) => *v,
                                _ => unreachable!("cant copy register to itself"),
                            }
                        }
                        Op::D => {
                            self.registers[3] = match value {
                                Op::B => self.registers[1],
                                Op::C => self.registers[2],
                                Op::A => self.registers[0],
                                Op::N(v) => *v,
                                _ => unreachable!("cant copy register to itself"),
                            }
                        }
                        _ => unreachable!("dst must be a register"),
                    },
                    Instruction::JNZ { cond, count } => match cond {
                        Op::A => {
                            if self.registers[0] != 0 {
                                self.pc = (self.pc as i32 + count - 1) as usize;
                            }
                        }
                        Op::B => {
                            if self.registers[1] != 0 {
                                self.pc = (self.pc as i32 + count - 1) as usize;
                            }
                        }
                        Op::C => {
                            if self.registers[2] != 0 {
                                self.pc = (self.pc as i32 + count - 1) as usize;
                            }
                        }
                        Op::D => {
                            if self.registers[3] != 0 {
                                self.pc = (self.pc as i32 + count - 1) as usize;
                            }
                        }
                        Op::N(v) => {
                            if *v != 0 {
                                self.pc = (self.pc as i32 + count - 1) as usize;
                            }
                        }
                    },
                }
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
        .expect("expecting input file name as cli argument");

    let program: Vec<Instruction> = Parser::new(&input_file)?.collect();

    let mut machine = Machine::new();
    machine.run(&program);
    println!("{}", machine.registers[0]);

    let mut machine = Machine::new();
    machine.registers[2] = 1;
    machine.run(&program);
    println!("{}", machine.registers[0]);

    Ok(())
}
