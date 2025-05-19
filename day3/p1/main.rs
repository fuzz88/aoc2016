use std::cmp;
use std::fs;

fn main() {
    println!("--- Day 3: Squares With Three Sides ---");

    println!(
        "{}",
        fs::read_to_string("input.txt")
            .unwrap()
            .lines()
            .map(|line| {
                let mut triangle: Vec<u32> = line
                    .split_whitespace()
                    .map(|side| side.parse::<u32>().unwrap())
                    .collect();

                triangle.sort();

                let max_side = triangle[2];
                let sum_of_less_sides = triangle[0] + triangle[1];

                match max_side.cmp(&sum_of_less_sides) {
                    cmp::Ordering::Less => 1,
                    _ => 0,
                }
            })
            .sum::<u32>()
    );
}
