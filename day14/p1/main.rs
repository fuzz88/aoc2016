#![feature(iter_map_windows)]
mod md5;
use md5::compute;

use std::collections::HashSet;

fn main() {
    println!("--- Day 14: One-Time Pad ---");

    let salt = "yjdafjpo";
    let mut count = 0;
    let mut idx = 0;

    let mut producers: Vec<u32> = vec![];

    let mut checking: Vec<(u8, u32)> = vec![];
    let mut idxs_to_remove: HashSet<u32> = HashSet::new();

    loop {
        let to_hash = [salt, &idx.to_string()].concat();
        let hash = compute(&to_hash);

        if let Some(ch) = check_5(&hash) {
            for check in &checking {
                if idx - check.1 < 1000 {
                    if ch == check.0 {
                        producers.push(check.1);
                        count += 1;
                    }
                } else {
                    idxs_to_remove.insert(check.1);
                }
            }
        }

        checking = checking
            .iter()
            .filter(|(_, idx)| !idxs_to_remove.contains(idx))
            .map(|(ch, idx)| (*ch, *idx))
            .collect();
        idxs_to_remove.clear();

        match find_3(&hash) {
            None => {}
            Some(ch) => {
                checking.push((ch, idx));
            }
        }

        if count > 64 {
            producers.sort();
            println!("{}", producers[63]);
            break;
        }

        idx += 1;
    }
}

fn find_3(hash: &[u8; 16]) -> Option<u8> {
    for i in 0..hash.len() - 1 {
        if hash[i] & 0x0F == hash[i + 1] >> 4 && hash[i] & 0x0F == hash[i] >> 4 {
            return Some(hash[i]);
        }
        if hash[i + 1] & 0x0F == hash[i + 1] >> 4 && hash[i] & 0x0F == hash[i + 1] >> 4 {
            return Some(hash[i + 1]);
        }
    }
    None
}

fn check_5(hash: &[u8; 16]) -> Option<u8> {
    for i in 0..hash.len() - 2 {
        if hash[i + 1] >> 4 == hash[i + 1] & 0x0F
            && ((hash[i + 1] == hash[i + 2] && hash[i + 1] >> 4 == hash[i] & 0x0F)
                || (hash[i] == hash[i + 1] && hash[i + 1] & 0x0F == hash[i + 2] >> 4))
        {
            return Some(hash[i + 1]);
        }
    }
    None
}
