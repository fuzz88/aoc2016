#![feature(ascii_char)]

fn main() {
    println!("--- Day 9: Explosives in Cyberspace ---");

    let input_file = std::env::args().nth(1).expect("expecting input file name");
    let input_data = read_input(&input_file);

    for line in input_data.lines() {
        println!("{}", decompressed_len(&line));
    }
}

fn read_input(filename: &str) -> String {
    std::fs::read_to_string(filename).expect("expecting valid input data")
}

fn decompressed_len(compressed: &str) -> u64 {
    let mut idx: usize = 0;
    let mut result: u64 = 0;
    let mut in_expr: bool = false;
    let mut expr_start: usize = 0;
    let mut expr_end;

    let compressed = compressed.replace(" ", "");
    let compressed = compressed
        .as_ascii()
        .expect("expecting input data to be ascii");

    loop {
        let ch: char = compressed[idx].into();

        match ch {
            '(' if !in_expr => {
                in_expr = true;
                expr_start = idx + 1;
                result += 1;
                idx += 1;
            }
            ')' if in_expr => {
                in_expr = false;
                expr_end = idx;
                result -= (idx - expr_start + 1) as u64;

                let expr = compressed[expr_start..expr_end].as_str();

                let mut components = expr.split("x");
                let (count, repeat) = (
                    components.next().unwrap().parse::<u64>().unwrap(),
                    components.next().unwrap().parse::<u64>().unwrap(),
                );

                let start = idx + 1;

                idx += count as usize + 1;

                result +=
                    repeat * decompressed_len(compressed[start..start + count as usize].as_str());
            }
            _ => {
                result += 1;
                idx += 1;
            }
        }

        if idx == compressed.len() {
            break result;
        }
    }
}
