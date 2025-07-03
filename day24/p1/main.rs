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

fn get_xy(col: i32, row: i32, map: &Map) -> Loc {
    map.field[col as usize + row as usize * map.cols]
}

fn calculate_poi_routes(col: i32, row: i32, map: &Map) -> Routes {
    let mut routes = HashMap::new();
    let mut visited = HashSet::<(i32, i32)>::new();
    let mut to_visit = VecDeque::<(i32, i32, u32)>::new();

    let ng = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

    visited.insert((col, row));
    to_visit.push_back((col, row, 0));

    loop {
        if let Some(next_loc) = to_visit.pop_front() {
            let x = next_loc.0;
            let y = next_loc.1;

            for idx in 0..4 {
                let dx = ng[idx].0;
                let dy = ng[idx].1;
                let nx = x + dx;
                let ny = y + dy;
                if nx > 0 && ny > 0 && ny < map.rows as i32 && nx < map.cols as i32 {
                    if !visited.contains(&(nx, ny)) {
                        match get_xy(nx as i32, ny as i32, map) {
                            Loc::Wall => {}
                            Loc::Open => {
                                visited.insert((nx, ny));
                                to_visit.push_back((nx, ny, next_loc.2));
                            }
                            Loc::POI(poi) => {
                                visited.insert((nx, ny));
                                to_visit.push_back((nx, ny, next_loc.2));
                                // add route length from (col, row) to this poi
                            }
                        }
                    }
                }
            }
        } else {
            break;
        }
    }

    routes
}

fn calculate_routes(map: &Map) -> Routes {
    let mut routes = HashMap::new();

    for col in 0..map.cols {
        for row in 0..map.rows {
            match get_xy(col as i32, row as i32, map) {
                Loc::POI(num) => {
                    let poi_routes: Routes = calculate_poi_routes(col as i32, row as i32, map);
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
