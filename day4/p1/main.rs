use std::collections::HashMap;
use std::env;
use std::fs;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

fn main() {
    println!("--- Day4: Security Through Obscurity ---");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("ERROR: no input file");
        return;
    }

    let input_file = &args[1];
    let input_data = read_input(input_file);

    println!("{}", solve_p1(&input_data));
    println!("{}", solve_p2(&input_data));
}

fn solve_p2(records: &Vec<Record>) -> u32 {
    for record in records {
        match sector_id_if_valid(record) {
            None => continue,
            Some(id) => {
                if decode_cipher(record).contains("northpole") {
                    return id;
                }
            }
        }
    }
    0
}

fn decode_cipher(record: &Record) -> String {
    let mut result = String::new();
    let name = &record.name;
    let sector_id = record.sector_id;

    for ch in name.chars() {
        if ch == '-' {
            result.push(' ');
            continue;
        }
        let decoded = ALPHABET.as_bytes()[((ch as usize - 97 + sector_id as usize) % 26) as usize];
        result.push(decoded as char);
    }
    result
}

fn solve_p1(records: &Vec<Record>) -> u32 {
    records
        .iter()
        .map(|record| match sector_id_if_valid(record) {
            Some(id) => id,
            None => 0,
        })
        .sum()
}

fn sector_id_if_valid(record: &Record) -> Option<u32> {
    if record.checksum == calculate_checksum(record) {
        return Some(record.sector_id);
    }
    None
}

fn calculate_checksum(record: &Record) -> String {
    let mut counter: HashMap<char, u32> = HashMap::new();

    for ch in record.name.chars() {
        if ch == '-' {
            continue;
        }
        let freq = counter.entry(ch).or_insert(0);
        *freq += 1;
    }

    let mut pairs: Vec<(&char, &u32)> = counter.iter().collect();

    pairs.sort_by(|a, b| a.0.cmp(&b.0));
    pairs.sort_by(|a, b| b.1.cmp(&a.1));

    let mut result = String::new();
    for ch in &pairs[..5] {
        result.push(*ch.0);
    }
    result
}

fn read_input(filename: &str) -> Vec<Record> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split('[').collect();
            let checksum: String = parts[1][..parts[1].len() - 1].to_string();

            let parts: (&str, &str) = parts[0].rsplit_once('-').unwrap();
            let name: String = parts.0.to_string();
            let sector_id: u32 = parts.1.parse().unwrap();

            Record {
                name,
                sector_id,
                checksum,
            }
        })
        .collect()
}

struct Record {
    name: String,
    sector_id: u32,
    checksum: String,
}
