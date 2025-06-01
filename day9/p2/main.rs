#![feature(ascii_char)]

fn main() {
    println!("--- Day 9: Explosives in Cyberspace ---");

    let input_file = std::env::args().nth(1).expect("expecting input file name");
    let input_data = read_input(&input_file);

    println!("{}", &input_data);
}

fn read_input(filename: &str) -> String {
    std::fs::read_to_string(filename).expect("expecting valid input data")
}
