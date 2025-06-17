use std::collections::VecDeque;

fn main() {
    println!("--- Day 19: An Elephant Named Joseph ---");

    let mut elf_count = 3018458;
    println!("{}", winner1(elf_count));
    println!("{}", winner2(elf_count));
}

fn winner1(n: u32) -> u32 {
    1 + (n - u32::pow(2, (n.ilog2() as f32).floor() as u32)) * 2
}

fn winner2(n: u32) -> u32 {
    let mut left: VecDeque<u32> = (1..=n/2).collect();
    let mut right: VecDeque<u32> = (n/2+1..=n).collect();

    while right.len() != 1 {
        right.pop_front();
        right.push_back(left.pop_front().unwrap());
        if right.len() - left.len() == 2 {
            left.push_back(right.pop_front().unwrap());
        }
    }
    left[0]
}
