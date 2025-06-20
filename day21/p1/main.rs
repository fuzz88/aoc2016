use std::env;
use std::error;
use std::fs;

#[derive(Debug, Clone)]
enum Op {
    SwapPos { x: usize, y: usize },
    SwapLetter { x: char, y: char },
    RotateLeft { x: usize },
    RotateRight { x: usize },
    RotateLetter { x: char },
    Reverse { x: usize, y: usize },
    Move { x: usize, y: usize },
}

impl From<&str> for Op {
    fn from(line: &str) -> Self {
        let components: Vec<&str> = line.split_whitespace().collect();

        match components[0] {
            "swap" => match components[1] {
                "position" => Op::SwapPos {
                    x: components[2].parse().unwrap(),
                    y: components[5].parse().unwrap(),
                },
                "letter" => Op::SwapLetter {
                    x: components[2].chars().nth(0).unwrap(),
                    y: components[5].chars().nth(0).unwrap(),
                },
                _ => unreachable!("no other variants of swap"),
            },
            "reverse" => Op::Reverse {
                x: components[2].parse().unwrap(),
                y: components[4].parse().unwrap(),
            },
            "rotate" => match components[1] {
                "left" => Op::RotateLeft {
                    x: components[2].parse().unwrap(),
                },
                "right" => Op::RotateRight {
                    x: components[2].parse().unwrap(),
                },
                "based" => Op::RotateLetter {
                    x: components[6].chars().nth(0).unwrap(),
                },
                _ => unreachable!("no other variants of rotate"),
            },
            "move" => Op::Move {
                x: components[2].parse().unwrap(),
                y: components[5].parse().unwrap(),
            },
            value => todo!("{}", value),
        }
    }
}

impl Op {
    fn apply(&self, text: &str) -> String {
        let mut letters: Vec<char> = text.chars().collect();

        match self {
            Op::SwapPos { x, y } => {
                letters.swap(*x, *y);
            }
            Op::SwapLetter { x, y } => {
                let x_pos = letters.iter().position(|letter| *letter == *x).unwrap();
                let y_pos = letters.iter().position(|letter| *letter == *y).unwrap();
                letters.swap(x_pos, y_pos);
            }
            Op::Reverse { x, y } => {
                letters[*x..=*y].reverse();
            }
            Op::RotateLeft { x } => {
                let length = letters.len();
                for _ in 0..*x {
                    let first = letters[0];
                    for i in 0..length - 1 {
                        letters[i] = letters[i + 1];
                    }
                    letters[length - 1] = first;
                }
            }
            Op::RotateRight { x } => {
                let length = letters.len();
                for _ in 0..*x {
                    let last = letters[length - 1];
                    for i in (1..length).rev() {
                        letters[i] = letters[i - 1];
                    }
                    letters[0] = last;
                }
            }
            Op::Move { x, y } => {
                let moved = letters.remove(*x);
                letters.insert(*y, moved);
            }
            Op::RotateLetter { x } => {
                // rotate to right
                let x_pos = letters.iter().position(|letter| *letter == *x).unwrap();
                let length = letters.len();
                for _ in 0..(1 + x_pos + if x_pos >= 4 { 1 } else { 0 }) {
                    let last = letters[length - 1];
                    for i in (1..length).rev() {
                        letters[i] = letters[i - 1];
                    }
                    letters[0] = last;
                }
            }
        }

        letters.iter().collect()
    }

    fn get_reverted(&self, text: &str) -> Option<Op> {
        match self {
            Op::RotateLeft { x } => Some(Op::RotateRight { x: *x }),
            Op::RotateRight { x } => Some(Op::RotateLeft { x: *x }),
            Op::Move { x, y } => Some(Op::Move { x: *y, y: *x }),
            Op::RotateLetter { x } => {
                let x_pos = text.find(*x).unwrap();
                let length = text.len();

                let mut solutions = vec![];

                for i in 0..length {
                    let shift = i + 1 + if i >= 4 { 1 } else { 0 };
                    if (i + shift) % length == x_pos {
                        solutions.push(shift);
                    }
                }

                if solutions.len() != 1 {
                    None
                } else {
                    Some(Op::RotateLeft { x: solutions[0] })
                }
            }
            op => Some(op.clone()),
        }
    }
}

fn read_input(filename: &str) -> Result<Vec<Op>, Box<dyn error::Error>> {
    let ops = fs::read_to_string(filename)?
        .lines()
        .map(|line| Op::from(line))
        .collect();

    Ok(ops)
}

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("--- Day 21: Scrambled Letters and Hash ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no input file as cli argument is provided")?;

    let mut text = env::args()
        .nth(2)
        .ok_or("no text for scrambler as cli argument is provided")?;

    let mode = env::args()
        .nth(3)
        .ok_or("no scrambler mode as cli argument is provided")?;

    let scrambler_function = read_input(&input_file)?;

    match mode.as_str() {
        "s" => {
            for operation in scrambler_function {
                text = operation.apply(&text);
                // println!("{:?} {}", operation, text);
            }
        }
        "u" => {
            for operation in scrambler_function.iter().rev() {
                text = operation
                    .get_reverted(&text)
                    .ok_or("ambiguous Op::RotateLetter")?
                    .apply(&text);
                // println!("{:?} {}", operation, text);
            }
        }
        _ => unreachable!("wrong mode"),
    }

    println!("{}", text);

    Ok(())
}
