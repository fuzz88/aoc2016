use md5::{compute};

mod md5;

fn main() {
    println!("--- Day 14: One-Time Pad ---");

    let salt = "yjdafjpo";

    println!("{:?}", compute(salt));
}
