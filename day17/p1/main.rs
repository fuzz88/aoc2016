use std::env;
use std::fs;

mod md5;
use md5::compute;

static mut shortest_len: usize = 1000000;
static mut shortest: String = String::new();
static mut longest_len: usize = 0;
static mut longest: String = String::new();

fn main() {
    println!("--- Day 17: Two Steps Forward ---");

    let content = fs::read_to_string(env::args().nth(1).expect("no input file as clie argument"))
        .expect("expecting valid input filename");
    let passcodes: Vec<&str> = content.lines().collect();

    // println!("{passcodes:#?}");
    for passcode in passcodes {
        solve(passcode, (0, 0));
        // println!("  {}:", passcode);
        unsafe {
            if shortest_len != 1000000 {
                println!("{}", &shortest[passcode.len()..]);
                shortest_len = 1000000;
            }
            if longest_len != 0 {
                // println!("{}", &longest[passcode.len()..]);
                longest_len -= passcode.len();
                println!("{longest_len}");
                longest_len = 0;
            }
        }
    }
}

fn solve(passcode: &str, position: (i8, i8)) {
    // println!("{passcode} {position:?}");

    if position.1 > 3 || position.1 < 0 || position.0 > 3 || position.0 < 0 {
        return;
    }

    if position == (3, 3) {
        unsafe {
            if passcode.len() < shortest_len {
                shortest_len = passcode.len();
                shortest = passcode.to_string();
            }
            if passcode.len() > longest_len {
                longest_len = passcode.len();
                longest = passcode.to_string();
            }
        }
        return;
    }

    let hash = compute(passcode);

    if hash[0] & 0x0F > 10 {
        let mut p = passcode.to_string();
        p.push('D');
        solve(&p, (position.0, position.1 + 1));
    }
    if hash[1] & 0x0F > 10 {
        let mut p = passcode.to_string();
        p.push('R');
        solve(&p, (position.0 + 1, position.1));
    }
    if hash[0] >> 4 > 10 {
        let mut p = passcode.to_string();
        p.push('U');
        solve(&p, (position.0, position.1 - 1));
    }
    if hash[1] >> 4 > 10 {
        let mut p = passcode.to_string();
        p.push('L');
        solve(&p, (position.0 - 1, position.1));
    }
}
