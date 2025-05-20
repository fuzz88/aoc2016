use lib::{compute, Digest};

mod lib;

fn main() {
    println!("--- Day 5: How About a Nice Game of Chess? ---");
    let door_id = String::from("ffykfhsq");
    //let door_id = String::from("abc");
    let mut password: u64 = 0;

    let mut idx = 0;
    let mut count = 8;

    loop {
        let mut to_hash = door_id.clone();
        to_hash.push_str(&idx.to_string());
        let hash = compute(to_hash);
        if good(&hash) {
            println!("{:x}", hash);
            password += hash[2] as u64 * u64::pow(16, count-1);
            count -= 1;
        }
        if count == 0 {
            break;
        }
        idx += 1;
    }
    println!("{:x}", password);
}

// digest with five leading zeroes
fn good(digest: &Digest) -> bool {
    digest[0] == 0 && digest[1] == 0 && digest[2] < 16
}
