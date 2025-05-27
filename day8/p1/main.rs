fn main() {
    println!("--- Day8: Two-Factor Authentication ---");

    let input_file = std::env::args()
        .nth(1)
        .expect("expecting input file as command line argument");

    let commands = read_input(&input_file);
}

fn read_input(filename: &str) -> Vec<Command> {
    vec![]
}

enum Command {
    Rect {
        width: u32,
        height: u32,
    },
    Rotate {
        direction: bool,
        index: u32,
        count: u32,
    },
}
