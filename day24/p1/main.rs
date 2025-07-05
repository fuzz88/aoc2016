#![feature(iter_map_windows)]
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

fn calculate_poi_routes(col: i32, row: i32, map: &Map, from_poi: u8) -> Routes {
    let mut routes = HashMap::new();
    let mut visited = HashSet::<(i32, i32)>::new();
    let mut to_visit = VecDeque::<(i32, i32, u32)>::new();

    let ng = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

    visited.insert((col, row));
    to_visit.push_back((col, row, 0)); // cow, row, dist

    while let Some(next_loc) = to_visit.pop_front() {
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
                        Loc::Wall => {
                            continue;
                        }
                        Loc::Open => {
                            visited.insert((nx, ny));
                            to_visit.push_back((nx, ny, next_loc.2 + 1));
                        }
                        Loc::POI(poi) => {
                            // add route length from (col, row) to this poi
                            let poi_route = routes.entry(from_poi).or_insert(HashMap::new());
                            poi_route.insert(poi, next_loc.2 + 1);

                            visited.insert((nx, ny));
                            to_visit.push_back((nx, ny, next_loc.2 + 1));
                        }
                    }
                }
            }
        }
    }

    routes
}

fn calculate_routes(map: &Map) -> Routes {
    let mut routes = HashMap::new();

    for col in 0..map.cols {
        for row in 0..map.rows {
            match get_xy(col as i32, row as i32, map) {
                Loc::POI(poi) => {
                    let poi_routes: Routes = calculate_poi_routes(col as i32, row as i32, map, poi);
                    poi_routes.iter().for_each(|(p, r)| {
                        routes.insert(*p, r.clone());
                    });
                }
                _ => {}
            }
        }
    }

    routes
}

// https://en.wikipedia.org/wiki/Heap%27s_algorithm
//
fn permutations<T: Copy>(v: &mut Vec<T>) -> Vec<Vec<T>> {
    let n = v.len();
    let mut results = vec![];
    let mut c = vec![];

    (0..n).for_each(|_| {
        c.push(0);
    });

    results.push(v.clone());

    let mut i = 1;

    while i < n {
        if c[i] < i {
            if i % 2 == 0 {
                let t = v[0];
                v[0] = v[i];
                v[i] = t;
            } else {
                let t = v[c[i]];
                v[c[i]] = v[i];
                v[i] = t;
            }
            results.push(v.clone());
            c[i] += 1;
            i = 1;
        } else {
            c[i] = 0;
            i += 1;
        }
    }

    results
}

fn find_shortest_path1(routes: &Routes) -> u32 {
    let mut pois = routes.keys().map(|p| *p).collect();

    let shortest_path = permutations(&mut pois)
        .iter()
        .filter(|p| p[0] == 0)
        .map(|path| {
            path.iter()
                .map_windows(|[p1, p2]| {
                    if let Some(dists) = routes.get(p1) {
                        if let Some(dist) = dists.get(p2) {
                            *dist
                        } else {
                            unreachable!("broken premutations");
                        }
                    } else {
                        unreachable!("broken permutations");
                    }
                })
                .sum()
        })
        .min();

    shortest_path.expect("non-empty iterators")
}

fn find_shortest_path2(routes: &Routes) -> u32 {
    let mut pois: Vec<u8> = routes.keys().map(|p| *p).collect();
    pois.push(0);

    let shortest_path = permutations(&mut pois)
        .iter()
        .filter(|p| p[0] == 0 && p[p.len() - 1] == 0)
        .map(|path| {
            path.iter()
                .map_windows(|[p1, p2]| {
                    if let Some(dists) = routes.get(p1) {
                        if let Some(dist) = dists.get(p2) {
                            *dist
                        } else {
                            unreachable!("broken permutations");
                        }
                    } else {
                        unreachable!("broken permutations");
                    }
                })
                .sum()
        })
        .min();

    shortest_path.expect("non-empty iterators")
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

    let shortest_path1 = find_shortest_path1(&routes);
    let shortest_path2 = find_shortest_path2(&routes);

    // println!("{routes:#?}");
    println!("{shortest_path1}");
    println!("{shortest_path2}");

    Ok(())
}
