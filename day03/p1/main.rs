use std::cmp;
use std::fs;

fn main() {
    println!("--- Day 3: Squares With Three Sides ---");

    let count: u32 = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let mut triangle: Vec<u32> = line
                .split_whitespace()
                .map(|side| side.parse().unwrap())
                .collect();

            triangle.sort();

            match triangle[2].cmp(&(triangle[0] + triangle[1])) {
                cmp::Ordering::Less => 1,
                _ => 0,
            }
        })
        .sum();

    println!("{}", count);
}
