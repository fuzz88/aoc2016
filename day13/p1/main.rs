use std::cmp::min;
use std::collections::{HashSet, VecDeque};

fn make_is_wall(dfn: i32) -> impl Fn(i32, i32) -> bool {
    move |x: i32, y: i32| {
        #[rustfmt::skip]
        let n = x*x + 3*x + 2*x*y + y + y*y;
        (n + dfn).count_ones() % 2 != 0
    }
}

fn search_path_and_count_dist(
    x: i32,
    y: i32,
    xd: i32,
    yd: i32,
    max_dist: u32,
    is_wall: impl Fn(i32, i32) -> bool,
) -> (u32, u32) {
    // bfs
    // hope it will stop on this infinite maze

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut for_processing: VecDeque<(i32, i32, u32)> = VecDeque::new();

    let mut min_dist = 1_000_000;
    let mut count = 0;

    for_processing.push_back((x, y, 0));
    visited.insert((x, y));

    while !for_processing.is_empty() {
        let Some((x, y, dist)) = for_processing.pop_front() else {
            unreachable!("queue is not empty")
        };

        if x == xd && y == yd {
            min_dist = min(dist, min_dist);
        }
        if dist <= max_dist {
            // println!("{} {}", x, y);
            count += 1;
        }

        let diffs = (-1..=1).flat_map(|i| (-1..=1).map(move |j| (i, j)));

        diffs
            .filter(|(dx, dy)| *dx == 0 || *dy == 0) // filter out diagonal diffs
            .map(|(dx, dy)| (x + dx, y + dy)) // apply diffs to get neighbours
            .filter(|(nx, ny)| {
                // filter out negatives and walls from neighbours
                *nx >= 0 && *ny >= 0 && !is_wall(*nx, *ny)
            })
            .for_each(|(nx, ny)| {
                if !visited.contains(&(nx, ny)) {
                    // filter out already visited
                    visited.insert((nx, ny));
                    for_processing.push_back((nx, ny, dist + 1));
                }
            });
    }

    (min_dist, count)
}

fn main() {
    println!("--- Day 13: A Maze of Twisty Little Cibicles ---");

    println!(
        "{:?}",
        search_path_and_count_dist(1, 1, 31, 39, 50, make_is_wall(1358))
    );
}
