use std::env;
use std::fs;

fn main() {
    println!("--- Day4: Security Through Obscurity ---");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("ERROR: no input file");
        return;
    }

    let input_file = &args[1];
    let input_data = read_input(input_file);
    dbg!(&input_data);
    println!("{}", input_data.len());
}

fn read_input(filename: &str) -> Vec<Record> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split('[').collect();
            let checksum: String = parts[1][..parts[1].len() - 1].to_string();

            let parts: (&str, &str) = parts[0].rsplit_once('-').unwrap();
            dbg!(&parts);
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

#[derive(Debug)]
struct Record {
    name: String,
    sector_id: u32,
    checksum: String,
}
