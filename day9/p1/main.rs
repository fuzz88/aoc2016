#![feature(ascii_char)]

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
    let mut idx: usize = 0;
    let mut result: u32 = 0;
    let mut in_exp: bool = false;
    let mut exp_start: usize = 0;
    let mut exp_end;

    let compressed = compressed.replace(" ", "");
    let compressed = compressed
        .as_ascii()
        .expect("expecting input data to be ascii");

    loop {
        let ch: char = compressed[idx].into();

        match ch {
            '(' => {
                in_exp = true;
                exp_start = idx + 1;
            }
            ')' => {
                in_exp = false;
                exp_end = idx - 1;
                let mut components = compressed[exp_start..=exp_end].as_str().split("x");
                let (count, repeat) = (
                    components.next().unwrap().parse::<u32>().unwrap(),
                    components.next().unwrap().parse::<u32>().unwrap(),
                );
                println!("{} {}", count, repeat);
                idx += count as usize;
                result += count * repeat;
            }
            _ => {
                if !in_exp {
                    result += 1;
                }
            }
        }

        idx += 1;
        if idx == compressed.len() - 1 {
            break result;
        }
    }
}
