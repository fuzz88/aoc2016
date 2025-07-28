use std::env;
use std::fs;
use std::process;

fn main() {
    println!("--- Day7: Internet Protocol Version 7 ---");

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("ERROR: no input file");
        process::exit(1);
    }

    let input_file = &args[1];
    let input_data = read_input(input_file);

    println!("{}", solve(&input_data));
}

fn read_input(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect()
}

fn solve(ips: &Vec<String>) -> u32 {
    ips.iter().map(|ip| check_tls(ip)).sum()
}

fn check_tls(ip: &str) -> u32 {
    let len = ip.len();
    let ip_bytes = ip.as_bytes();
    let mut in_brackets: u32 = 0;
    let mut idx: usize = 0;
    let mut found: bool = false;
    loop {
        if idx >= len - 3 {
            break; // the end of the string
        }
        if ip_bytes[idx + 3] == b'[' || ip_bytes[idx + 3] == b']' {
            idx += 3; // if bracket ahead jump to it
        }
        if ip_bytes[idx] == b'[' {
            in_brackets += 1;
            idx += 1;
        }
        if ip_bytes[idx] == b']' {
            in_brackets -= 1;
            idx += 1;
        }
        if found && in_brackets == 0 {
            idx += 1;
            continue; // if already found ABBA outside brackets, check only inside brackets
        }
        if check_pairs(&ip_bytes[idx..idx + 4]) {
            found = true;
            if in_brackets != 0 {
                return 0;
            }
        }
        idx += 1;
    }
    if found {
        return 1;
    } else {
        return 0;
    }
}

fn check_pairs(b: &[u8]) -> bool {
    //b.into_iter().for_each(|el| print!("{}", *el as char));
    //println!();
    b[0] == b[3] && b[1] == b[2] && b[0] != b[1]
}
