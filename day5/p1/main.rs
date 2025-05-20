use md5::{compute, Digest};
use std::io::{stdout, Write};

mod md5;

fn main() {
    println!("--- Day 5: How About a Nice Game of Chess? ---");

    let door_id = String::from("ffykfhsq");

    let mut password: u64 = 0;
    let mut second_password: u64 = 0;
    let mut digits = [false; 8];

    let mut idx = 0;
    let mut count = 8;
    let mut second_count = 0;

    loop {
        let mut to_hash = door_id.clone();
        to_hash.push_str(&idx.to_string());
        let hash = compute(to_hash);
        if good(&hash) {
            println!("{:x}", hash);
            if count != 0 {
                password += hash[2] as u64 * u64::pow(16, count - 1);
                count -= 1;
                println!(".");
            }
            if second_count != 8 {
                let digit = hash[3] >> 4;
                let place = hash[2] as usize;
                if place < 8 && !digits[place] {
                    digits[place] = true;
                    second_password += digit as u64 * u64::pow(16, 7 - place as u32);
                    second_count += 1;
                    println!(".");
                }
            }
            if second_count == 8 {
                break;
            }
        }
        idx += 1;
    }
    println!("{:08x}", password);
    println!("{:08x}", second_password);
}

// digest with five leading zeroes
fn good(digest: &Digest) -> bool {
    digest[0] == 0 && digest[1] == 0 && digest[2] < 16
}
