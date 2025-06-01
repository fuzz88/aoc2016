fn main() {
    println!("--- Day 9: Explosives in Cyberspace ---");

    let input_file = std::env::args().nth(1).expect("expecting input file name");
    let input_data = read_input(&input_file);

    println!("{}", decompressed_len(&input_data));
}

fn read_input(filename: &str) -> String {
    std::fs::read_to_string(filename).expect("expecting valid input data")
}

fn decompressed_len(compressed: &str) -> u32 {
    compressed.len() as u32
}
