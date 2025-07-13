use std::env;
use std::error::Error;
use std::fs;
use std::process;

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
enum Op {
    Register(Reg),
    Number(i32),
}

#[derive(Debug, Clone)]
enum Instruction {
    CPY { value: Op, dst: Op },
    INC(Op),
    DEC(Op),
    JNZ { cond: Op, count: Op },
    TGL(Op),
    OUT(Op),
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
                "tgl" => Instruction::TGL(self.parse_next_token_as_operand()),
                "out" => Instruction::OUT(self.parse_next_token_as_operand()),
                unknown => todo!("parsed unknown instruction: {:?}", unknown),
            })
        }
    }
}

struct Machine {
    registers: [i32; 4],
    pc: usize,
    out: i32,
    out_success: u32,
}

impl Machine {
    fn new() -> Self {
        Machine {
            registers: [0; 4],
            pc: 0,
            out: 0,
            out_success: 512, // maybe enough
        }
    }

    fn check_out(&mut self, x: i32) -> bool {
        let result = self.out == x;
        if self.out == 0 {
            self.out = 1;
        } else {
            self.out = 0;
        }
        result
    }

    fn run(&mut self, program: &mut Vec<Instruction>) {
        while self.pc < program.len() {
            match &program[self.pc] {
                Instruction::OUT(op) => {
                    if self.check_out(match op {
                        Op::Number(n) => *n,
                        Op::Register(reg) => self.registers[reg.idx()],
                    }) {
                        self.out_success -= 1;
                        if self.out_success == 0 {
                            println!("success");
                            process::exit(0);
                        }
                    } else {
                        break;
                    }
                }
                Instruction::TGL(op) => {
                    let tgl_jmp = match op {
                        Op::Number(n) => *n,
                        Op::Register(reg) => self.registers[reg.idx()],
                    };
                    let tgl_dst = (self.pc as i32 + tgl_jmp) as usize;
                    if tgl_dst > 0 && tgl_dst < program.len() {
                        let to_patch = &program[tgl_dst];

                        program[tgl_dst] = match to_patch {
                            Instruction::INC(op) => Instruction::DEC(*op),
                            Instruction::DEC(op) => Instruction::INC(*op),
                            Instruction::TGL(op) => Instruction::INC(*op),
                            Instruction::CPY { value: x, dst: y } => Instruction::JNZ {
                                cond: *x,
                                count: *y,
                            },
                            Instruction::JNZ { cond: x, count: y } => {
                                Instruction::CPY { value: *x, dst: *y }
                            }
                            Instruction::OUT(op) => Instruction::INC(*op),
                        }
                    }
                }
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
                            let count = match count {
                                Op::Number(count) => *count,
                                Op::Register(reg) => self.registers[reg.idx()],
                            };
                            self.pc = (self.pc as i32 + count - 1) as usize;
                        };
                    }
                    Op::Number(value) => {
                        // println!("jnz {}", *value);
                        if *value != 0 {
                            let count = match count {
                                Op::Number(count) => *count,
                                Op::Register(reg) => self.registers[reg.idx()],
                            };
                            self.pc = (self.pc as i32 + count - 1) as usize;
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
    println!("--- Day 25: Clock Signal ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("expecting input file name as cli argument")?;

    let mut program = Parser::tokenize(&input_file)?.collect::<Vec<Instruction>>();

    // println!("{:#?}", program);

    for i in 0..1024 {
        println!("{}", i);
        let mut machine = Machine::new();
        machine.registers[Reg::A.idx()] = i;
        machine.run(&mut program);
    }

    // guess it's better to analyze input.txt program.
    // but this solution is ok, because we can check answer anyway.
    Ok(())
}
