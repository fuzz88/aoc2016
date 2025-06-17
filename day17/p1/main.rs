use std::env;
use std::fs;
use std::process;

mod md5;
use md5::compute;

fn main() {
    println!("--- Day 17: Two Steps Forward ---");

    let content = fs::read_to_string(env::args().nth(1).expect("no input file as clie argument"))
        .expect("expecting valid input filename");
    let passcodes: Vec<&str> = content.lines().collect();

    println!("{passcodes:#?}");
}

fn solve(passcode: &str, position: (i8, i8)) {
    // println!("{passcode} {position:?}");

    if position.1 > 3 || position.1 < 0 || position.0 > 3 || position.0 < 0 {
        return;
    }

    if position == (3, 3) {
        println!("{passcode}");
        process::exit(1);
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
