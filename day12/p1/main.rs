use std::env;
use std::error::Error;
use std::fs;

#[derive(Debug)]
enum Op {
    A,
    B,
    C,
    D,
    N(u32),
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
            .map(|instruction| instruction.to_string())
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

fn main() -> Result<(), Box<dyn Error>> {
    println!("--- Day 12: Leonardo's Monorail ---");

    let input_file = env::args()
        .nth(1)
        .expect("expecting input file name as cli argument");

    let parser = Parser::new(&input_file)?;

    let program: Vec<Instruction> = parser.collect();

    println!("{:#?}", program);

    Ok(())
}
