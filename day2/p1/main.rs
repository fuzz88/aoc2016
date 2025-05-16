use std::convert::TryInto;
use std::fs::read_to_string;

fn main() {
    println!("--- Day 2: Bathroom Security ---");

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("ERROR: no input file");
        return;
    }
    let input_filename = &args[1];

    solve_part1(input_filename);
}

fn solve_part1(filename: &str) -> u32 {
    let curr_pos = Pos {
        coords: (1, 1),
        digit: 5,
    };
    dbg!(read_input(filename)
        .iter()
        .rev()
        .enumerate()
        .map(|(line_num, instr_line)| {
            let power: u32 = line_num.try_into().unwrap();
            dbg!(&curr_pos);
            1 * u32::pow(10, power)
        })
        .sum::<u32>());
    todo!("solve");
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
    coords: (u8, u8),
    digit: u8,
}

#[derive(Debug)]
enum Instr {
    Up,
    Down,
    Left,
    Right,
}
