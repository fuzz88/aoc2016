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
    println!("{}", input_file);
    println!("{:?}", input_data);
}

fn read_input(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect()
}
