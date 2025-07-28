use std::fs;

fn main() {
    println!(" --- Day3: Squares With Three Sides --- ");

    let mut count_possible: u32 = 0;
    let mut idx = 0;
    let mut triplet = [[0; 3], [0; 3], [0; 3]];

    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let nums: Vec<u32> = line
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        triplet[0][idx] = nums[0];
        triplet[1][idx] = nums[1];
        triplet[2][idx] = nums[2];

        if idx < 2 {
            idx += 1;
        } else {
            idx = 0;
            for mut triangle in triplet {
                triangle.sort();
                if triangle[0] + triangle[1] > triangle[2] {
                    count_possible += 1;
                }
            }
        }
    }
    println!("{}", count_possible);
}
