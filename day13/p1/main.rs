use std::collections::{HashSet, VecDeque};

#[rustfmt::skip]
fn make_is_wall(dfn: i32) -> impl Fn(i32, i32) -> bool {
    move |x: i32, y: i32| {
        let n = x*x + 3*x + 2*x*y + y + y*y;
        (n + dfn).count_ones() % 2 != 0
    }
}

fn search_path(x: i32, y: i32, xd: i32, yd: i32, is_wall: impl Fn(i32, i32) -> bool) {
    // bfs
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut for_processing: VecDeque<(i32, i32, u32)> = VecDeque::new();

    for_processing.push_back((x, y, 0));

    while !for_processing.is_empty() {
        let Some((x, y, dist)) = for_processing.pop_front() else {
            unreachable!("queue is not empty")
        };

        if x == xd && y == yd {
            println!("{}", dist);
            break;
        }

        visited.insert((x, y));

        let dxs = (-1..=1).flat_map(|i| (-1..=1).map(move |j| (i, j)));

        dxs.filter(|(dx, dy)| *dx == 0 || *dy == 0) // no diagonal movement
            .map(|(dx, dy)| (x + dx, y + dy)) // neighbours
            .filter(|(nx, ny)| {
                // filter out negatives, walls and visited
                *nx >= 0 && *ny >= 0 && !is_wall(*nx, *ny) && !visited.contains(&(*nx, *ny))
            })
            .for_each(|(nx, ny)| for_processing.push_back((nx, ny, dist + 1)));
    }
}

fn main() {
    println!("--- Day 13: A Maze of Twisty Little Cibicles ---");

    search_path(1, 1, 31, 39, make_is_wall(1358));
    // search_path(1, 1, 7, 4, make_is_wall(10));
}
