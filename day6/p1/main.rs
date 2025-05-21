use std::env;
use std::fs;
use std::process;

fn main() {
    println!("--- Day 6: Signals and Noise --- ");

    if let Some(input_file) = env::args().nth(1) {
        let input_data = read_input(&input_file);
        let message = decode(&input_data);
        println!("{}", message);
    } else {
        eprintln!("ERROR: no input file");
        process::exit(1);
    }
}

fn read_input(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|message| message.to_string())
        .collect()
}

fn decode(_messages: &Vec<String>) -> String {
    "123".to_string()
}
