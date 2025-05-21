use std::{collections::HashMap, env, fs, process};

fn main() {
    println!("--- Day 6: Signals and Noise --- ");

    if let Some(input_file) = env::args().nth(1) {
        let input_data = read_input(&input_file);
        let messages = decode(&input_data);

        println!("{}", messages.0);
        println!("{}", messages.1);
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

fn decode(messages: &Vec<String>) -> (String, String) {
    let mut counters: Vec<HashMap<char, u32>> = vec![];

    for message in messages {
        for (idx, ch) in message.chars().enumerate() {
            if counters.len() < idx + 1 {
                counters.push(HashMap::new());
            }
            let freq = counters[idx].entry(ch).or_insert(0);
            *freq += 1;
        }
    }

    let mut results = (String::new(), String::new());

    for counter in counters {
        let (most_common_char, _) = counter
            .iter()
            .max_by_key(|(_a, b)| **b)
            .expect("expecting not empty iterator");

        results.0.push(*most_common_char);

        let (least_common_char, _) = counter
            .iter()
            .min_by_key(|(_a, b)| **b)
            .expect("expecting not empty iterator");

        results.1.push(*least_common_char);
    }

    results
}
