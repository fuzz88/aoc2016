use std::collections::HashSet;
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
    ips.iter().map(|ip| check_ssl(ip)).sum()
}

fn check_ssl(ip: &str) -> u32 {
    //println!("-----");
    let mut in_brackets: Vec<String> = vec![];
    let mut out_brackets: Vec<String> = vec![];

    let ip_bytes = ip.as_bytes();
    let mut idx: usize = 0;
    let mut last: usize = 0;

    // devide ip into two arrays of strings
    loop {
        if ip_bytes[idx] == b'[' && idx != last {
            out_brackets.push(ip[last..idx].to_string());
            last = idx + 1;
        }
        if ip_bytes[idx] == b']' {
            in_brackets.push(ip[last..idx].to_string());
            last = idx + 1;
        }
        idx += 1;
        if idx == ip.len() {
            out_brackets.push(ip[last..idx].to_string());
            break;
        }
    }

    let mut matches: HashSet<(u8, u8)> = HashSet::new();
    for s in out_brackets {
        //sliding window for out of bracket strings
        for w in s.as_bytes().windows(3) {
            if let Some(m) = check_aba(w) {
                matches.insert(m);
            }
        }
    }
    //println!("{:?}", matches);

    if matches.iter().any(|m| {
        for s in &in_brackets {
            //wtf? how chain this to line below?
            if s.as_bytes().windows(3).any(|w| check_bab(w, *m)) {
                return true;
            }
        }
        false
    }) {
        //println!("supports");
        1
    } else {
        //println!("not supports");
        0
    }
}

fn check_aba(b: &[u8]) -> Option<(u8, u8)> {
    match b[0] == b[2] && b[0] != b[1] {
        true => Some((b[0], b[1])),
        false => None,
    }
}

fn check_bab(b: &[u8], p: (u8, u8)) -> bool {
    //println!("{:?}", p);
    //println!("{:?}", b);
    b[0] == b[2] && b[0] == p.1 && b[1] == p.0
}
