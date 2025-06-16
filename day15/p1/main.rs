use std::env;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    println!("--- Day 15: Timing is Everything ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no input file as a cli argument is provided")?;

    // println!("{input_file}");

    let mut input_data = read_input(&input_file)?;

    // println!("{input_data:#?}");

    let p1 = solve(&input_data);

    println!("{p1}");

    let disk_count = input_data.len();

    input_data.push(Disk {
        position_count: 11,
        reached_at: (disk_count + 1) as u32,
        starting_position: 0,
    });

    let p2 = solve(&input_data);

    println!("{p2}");

    Ok(())
}

fn solve(disks: &Vec<Disk>) -> u32 {
    let mut push_time = 0;
    loop {
        if disks.iter().all(|disk| {
            (disk.starting_position + disk.reached_at + push_time) % disk.position_count == 0
        }) {
            return push_time;
        }
        push_time += 1;
    }
}

#[derive(Debug)]
struct Disk {
    position_count: u32,
    reached_at: u32,
    starting_position: u32,
}

fn parse_disk(line: &str) -> Disk {
    let components: Vec<&str> = line.split_whitespace().collect();

    let position_count = components[3].parse().unwrap();
    let starting_position = components[11][..components[11].len() - 1].parse().unwrap();
    let reached_at = components[1][1..].parse().unwrap();

    Disk {
        position_count,
        reached_at,
        starting_position,
    }
}

fn read_input(filename: &str) -> Result<Vec<Disk>, Box<dyn Error>> {
    let disks = fs::read_to_string(filename)?
        .lines()
        .map(|line| parse_disk(line))
        .collect();

    Ok(disks)
}
