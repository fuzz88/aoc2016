#![feature(iter_array_chunks)]
use std::env;
use std::fs;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    println!("--- Day 16: Dragon Checksum ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no input file is provided as cli argument")?;

    let input_data = read_input(&input_file)?;

    let checksum = calculate_checksum(input_data.0, input_data.1);

    println!("{checksum}");

    Ok(())
}

fn calculate_checksum(disk_length: usize, mut initial_state: String) -> String {
    let mut result = String::new();

    loop {
        let a = initial_state;

        result.push_str(&a);
        result.push('0');

        for ch in a.chars().rev() {
            result.push(match ch {
                '0' => '1',
                '1' => '0',
                _ => unreachable!("input consists of 0 and 1"),
            });
        }

        if result.len() >= disk_length {
            break;
        } else {
            initial_state = result.clone();
            result.clear();
        };
    }

    result = result[..disk_length].to_string();
    let mut checksum: String;

    loop {
        checksum = result
            .chars()
            .array_chunks()
            .map(|[a, b]| match a == b {
                true => '1',
                false => '0',
            })
            .collect();

        if checksum.len() % 2 != 0 {
            break;
        }
        result = checksum;
    }

    checksum
}

fn read_input(filename: &str) -> Result<(usize, String)> {
    let content = fs::read_to_string(filename)?;
    let components: Vec<&str> = content.split_whitespace().collect();

    Ok((components[0].parse().unwrap(), components[1].to_string()))
}
