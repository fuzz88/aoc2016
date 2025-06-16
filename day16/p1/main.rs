use std::env;
use std::fs;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    println!("--- Day 16: Dragon Checksum ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no input file is provided as cli argument")?;

    let input_data = read_input(&input_file);

    println!("{input_data:?}");

    Ok(())
}

fn read_input(filename: &str) -> Result<(usize, String)> {
    let content = fs::read_to_string(filename)?;
    let components: Vec<&str> = content.split_whitespace().collect();

    Ok((components[0].parse().unwrap(), components[1].to_string()))
}
