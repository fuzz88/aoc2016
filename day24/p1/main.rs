use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::error;
use std::fs;

#[derive(Debug, Clone, Copy)]
enum Loc {
    Wall,
    Open,
    POI(u8),
}

type Field = Vec<Loc>;
type Routes = HashMap<u8, HashMap<u8, u32>>;

#[derive(Debug)]
struct Map {
    field: Field,
    rows: usize,
    cols: usize,
}

fn parse_line(line: &str) -> Vec<Loc> {
    line.chars()
        .map(|c| match c {
            '#' => Loc::Wall,
            '.' => Loc::Open,
            poi => Loc::POI(poi.to_string().parse().unwrap()),
        })
        .collect()
}

fn read_input(filename: &str) -> Result<Field, Box<dyn error::Error>> {
    let locations = fs::read_to_string(filename)?
        .lines()
        .map(|line| parse_line(line))
        .flatten()
        .collect();

    Ok(locations)
}

fn get_xy(row: usize, col: usize, map: &Map) -> Loc {
    map.field[col + row * map.cols]
}

fn calculate_poi_routes(row: usize, col: usize, map: &Map) -> Routes {
    let mut routes = HashMap::new();
    let mut visited = HashSet::<(usize, usize)>::new();
    let mut to_visit = VecDeque::<(usize, usize, u32)>::new();

    visited.insert((row, col));
    to_visit.push_back((row, col, 0));

    loop {
        if let Some(next_loc) = to_visit.pop_front() {
        } else {
            break;
        }
    }

    routes
}

fn calculate_routes(map: &Map) -> Routes {
    let mut routes = HashMap::new();

    for row in 0..map.rows {
        for col in 0..map.cols {
            match get_xy(row, col, map) {
                Loc::POI(num) => {
                    let poi_routes: Routes = calculate_poi_routes(row, col, map);
                    println!("{poi_routes:?}");
                }
                _ => {}
            }
        }
    }

    routes
}

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("--- Day 24: Air Duct Spelunking ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no file name as cli argument is provided")?;

    // println!("{input_file}");

    let input_data = read_input(&input_file)?;
    let rows = fs::read_to_string(&input_file)?.lines().count();
    let cols = input_data.len() / rows;
    let map = Map {
        field: input_data,
        rows,
        cols,
    };
    // println!("{map:?}");

    let routes = calculate_routes(&map);

    println!("{routes:?}");

    Ok(())
}
