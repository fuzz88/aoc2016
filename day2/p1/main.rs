use std::convert::TryInto;
use std::env;
use std::fs::read_to_string;

fn main() {
    println!("--- Day 2: Bathroom Security ---");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("ERROR: no input file");
        return;
    }
    let input_filename = &args[1];
    println!(
        "{}",
        solve(
            input_filename,
            next_position_p1,
            Pos {
                x: 1,
                y: 1,
                digit: 5
            },
            10
        )
    );
    println!(
        "{:x}",
        solve(
            input_filename,
            next_position_p2,
            Pos {
                x: 0,
                y: 2,
                digit: 5
            },
            16
        )
    );
}

fn solve(filename: &str, next_position: fn(&Pos, &Instr) -> Pos, start_pos: Pos, base: u32) -> u32 {
    let mut curr_pos = start_pos;
    let instr_lines = read_input(filename);
    let instr_count = instr_lines.len();
    let mut curr_power = instr_count;
    instr_lines
        .iter()
        .map(|instr_line| {
            let mut instructions = instr_line.iter();
            while let Some(next_instr) = instructions.next() {
                curr_pos = next_position(&curr_pos, next_instr);
            }
            curr_power -= 1;
            curr_pos.digit * u32::pow(base, curr_power.try_into().unwrap())
        })
        .sum::<u32>()
}

fn next_position_p2(p: &Pos, instr: &Instr) -> Pos {
    match instr {
        Instr::Up if p.y != 0 && (p.y >= 3 || (p.x <= p.y + 1 && p.x >= 3 - p.y)) => Pos {
            x: p.x,
            y: p.y - 1,
            digit: match p.y {
                1 | 4 => p.digit - 2,
                2 | 3 => p.digit - 4,
                _ => unreachable!("ERROR: wrong y"),
            },
        },
        Instr::Down if p.y != 4 && (p.y <= 1 || (p.x <= 5 - p.y && p.x >= p.y - 1)) => Pos {
            x: p.x,
            y: p.y + 1,
            digit: match p.y {
                0 | 3 => p.digit + 2,
                1 | 2 => p.digit + 4,
                _ => unreachable!("ERROR: wrong y"),
            },
        },
        Instr::Left if p.x >= i8::abs(p.y - 2) + 1 => Pos {
            x: p.x - 1,
            y: p.y,
            digit: p.digit - 1,
        },
        Instr::Right if p.x <= 3 - i8::abs(p.y - 2) => Pos {
            x: p.x + 1,
            y: p.y,
            digit: p.digit + 1,
        },
        _ => Pos {
            x: p.x,
            y: p.y,
            digit: p.digit,
        },
    }
}

fn next_position_p1(p: &Pos, instr: &Instr) -> Pos {
    match instr {
        Instr::Up if p.y != 0 => Pos {
            x: p.x,
            y: p.y - 1,
            digit: p.digit - 3,
        },
        Instr::Down if p.y != 2 => Pos {
            x: p.x,
            y: p.y + 1,
            digit: p.digit + 3,
        },
        Instr::Left if p.x != 0 => Pos {
            x: p.x - 1,
            y: p.y,
            digit: p.digit - 1,
        },
        Instr::Right if p.x != 2 => Pos {
            x: p.x + 1,
            y: p.y,
            digit: p.digit + 1,
        },
        _ => Pos {
            x: p.x,
            y: p.y,
            digit: p.digit,
        },
    }
}

fn read_input(filename: &str) -> Vec<Vec<Instr>> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    'U' => Instr::Up,
                    'D' => Instr::Down,
                    'L' => Instr::Left,
                    'R' => Instr::Right,
                    _ => unreachable!("ERROR: unexpected character in input"),
                })
                .collect()
        })
        .collect()
}

#[derive(Debug)]
struct Pos {
    x: i8,
    y: i8,
    digit: u32,
}

#[derive(Debug)]
enum Instr {
    Up,
    Down,
    Left,
    Right,
}
