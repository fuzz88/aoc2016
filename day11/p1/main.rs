use std::env;
use std::fs;

type Result<T> = std::result::Result<T, String>;

fn main() -> Result<()> {
    println!("--- Day 11: Radioisotope Thermoelectric Generators ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no input filename as cli argument")?;

    println!("{}", input_file);

    let input_data = read_input(&input_file);

    println!("{:?}", input_data);

    Ok(())
}

fn read_input(filename: &str) -> Result<Vec<String>> {
    Ok(fs::read_to_string(filename)
        .map_err(|err| format!("can't read from input file {filename}: {err}"))?
        .lines()
        .map(|line| line.to_owned())
        .collect())
}
