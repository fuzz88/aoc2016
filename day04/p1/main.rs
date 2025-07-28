// this program assumes that secotr_id is never equals 0.
// 0 is used for a "record was not found" situation.

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

fn solve_p2(records: &Vec<RoomRecord>) -> u32 {
    // What is the sector ID of the room where North Pole objects are stored?
    if let Some(found) = records
        .iter()
        .find(|record| match sector_id_if_valid(record) {
            None => false,
            Some(_) => decode_cipher(record).contains("northpole"),
        })
    {
        found.sector_id
    } else {
        0
    }
}

fn decode_cipher(record: &RoomRecord) -> String {
    let mut result = String::new();
    let name = &record.name;
    let sector_id = record.sector_id;

    for ch in name.chars() {
        result.push(match ch {
            '-' => ' ',
            ch => {
                let idx = ch as usize - 97;
                let shift = sector_id as usize;
                ALPHABET.as_bytes()[(idx + shift) % ALPHABET.len()] as char
            }
        });
    }
    result
}
fn solve_p1(records: &Vec<RoomRecord>) -> u32 {
    // What is the sum of the sector IDs of the real rooms?
    records
        .iter()
        .map(|record| match sector_id_if_valid(record) {
            Some(id) => id,
            None => 0,
        })
        .sum()
}

fn sector_id_if_valid(record: &RoomRecord) -> Option<u32> {
    match record.checksum == calculate_checksum(record) {
        true => Some(record.sector_id),
        false => None,
    }
}

fn calculate_checksum(record: &RoomRecord) -> String {
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
    // checksum is a top five character frequencies sorted alphabetically.
    for ch in &pairs[..5] {
        result.push(*ch.0);
    }
    result
}

fn read_input(filename: &str) -> Vec<RoomRecord> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split('[').collect();
            let checksum: String = parts[1][..parts[1].len() - 1].to_string();

            let parts: (&str, &str) = parts[0].rsplit_once('-').unwrap();
            let name: String = parts.0.to_string();
            let sector_id: u32 = parts.1.parse().unwrap();

            RoomRecord {
                name,
                sector_id,
                checksum,
            }
        })
        .collect()
}

struct RoomRecord {
    name: String,
    sector_id: u32,
    checksum: String,
}
