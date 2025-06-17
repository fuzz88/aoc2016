#![feature(iter_map_windows)]
use std::env;
use std::error;
use std::fs;

#[derive(Debug, Clone)]
enum Tile {
    Safe,
    Trap,
}

fn read_input(filename: &str) -> Result<Vec<Tile>, Box<dyn error::Error>> {
    let tiles = fs::read_to_string(filename)?
        .trim_end()
        .chars()
        .map(|ch| match ch {
            '.' => Tile::Safe,
            '^' => Tile::Trap,
            _ => unreachable!("no other variants"),
        })
        .collect();

    Ok(tiles)
}

fn get_next_row(row: &mut Vec<Tile>) -> Vec<Tile> {
    // println!("{row:?}");
    row.insert(0, Tile::Safe);
    row.push(Tile::Safe);

    row.iter()
        .map_windows(|[left, center, right]| match (left, center, right) {
            (Tile::Trap, Tile::Trap, Tile::Safe)
            | (Tile::Safe, Tile::Trap, Tile::Trap)
            | (Tile::Trap, Tile::Safe, Tile::Safe)
            | (Tile::Safe, Tile::Safe, Tile::Trap) => Tile::Trap,
            (_, _, _) => Tile::Safe,
        })
        .collect()
}

fn get_total_safe_count(n: usize, init_tile: &Vec<Tile>) -> usize {
    let mut count = 1;
    let mut safe_count = get_safe_count(init_tile);
    let mut current_tile = init_tile.clone();
    loop {
        count += 1;

        let next_tile = get_next_row(&mut current_tile);
        safe_count += get_safe_count(&next_tile);

        current_tile = next_tile;

        if count == n {
            break;
        }
    }
    safe_count
}

fn get_safe_count(tiles: &Vec<Tile>) -> usize {
    tiles
        .iter()
        .map(|tile| match tile {
            Tile::Trap => 0,
            Tile::Safe => 1,
        })
        .sum()
}

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("--- Day 18: Like a Rogue ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no input file as cli argument is provided")?;

    // println!("{input_file}");

    let input_data = read_input(&input_file)?;

    // println!("{input_data:#?}");

    let total_safe_count = get_total_safe_count(40, &input_data);
    println!("{total_safe_count}");

    let total_safe_count = get_total_safe_count(400000, &input_data);
    println!("{total_safe_count}");

    Ok(())
}
