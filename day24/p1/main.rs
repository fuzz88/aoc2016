use std::env;
use std::error;
use std::fs;

#[derive(Debug)]
enum Loc {
    Wall,
    Open,
    POI(u8),
}

type Map = Vec<Loc>;

fn parse_line(line: &str) -> Vec<Loc> {
    line.chars()
        .map(|c| match c {
            '#' => Loc::Wall,
            '.' => Loc::Open,
            poi => Loc::POI(poi.to_string().parse().unwrap()),
        })
        .collect()
}

fn read_input(filename: &str) -> Result<Map, Box<dyn error::Error>> {
    let locations = fs::read_to_string(filename)?
        .lines()
        .map(|line| parse_line(line))
        .flatten()
        .collect();

    Ok(locations)
}

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("--- Day 24: Air Duct Spelunking ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no file name as cli argument is provided")?;

    println!("{input_file}");

    let input_data = read_input(&input_file)?;
    let rows = fs::read_to_string(&input_file)?.lines().count();
    let cols = input_data.len() / rows;

    println!("{input_data:?}");
    println!("{cols} {rows}");

    Ok(())
}
