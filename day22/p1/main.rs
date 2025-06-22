use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::error;
use std::fs;

#[derive(Debug)]
struct Node {
    x: usize,
    y: usize,
    size: usize,
    used: usize,
}

impl From<&str> for Node {
    fn from(line: &str) -> Self {
        let components: Vec<&str> = line.split_whitespace().collect();
        let node_path: Vec<&str> = components[0].split('-').collect();

        let x = node_path[1][1..].parse().unwrap();
        let y = node_path[2][1..].parse().unwrap();

        let size = components[1][..components[1].len() - 1].parse().unwrap();
        let used = components[2][..components[2].len() - 1].parse().unwrap();

        Node { x, y, size, used }
    }
}

type Graph = HashMap<(usize, usize), (usize, usize)>;

fn build_graph(nodes: &Vec<Node>) -> Graph {
    let mut graph = HashMap::new();

    for node in nodes {
        graph.insert((node.x, node.y), (node.size, node.used));
    }

    graph
}

fn get_neighbours(graph: &Graph, x_y: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbours = vec![];
    let x = x_y.0 as i32;
    let y = x_y.1 as i32;

    // println!("{:?}", x_y);
    // is it cleaner as iterators and flat_map? see day13
    for dx in -1..=1 {
        for dy in -1..=1 {
            if !(dx == 0 && dy == 0)
                && (dx == 0 || dy == 0)
                && x + dx >= 0
                && y + dy >= 0
                && graph.contains_key(&((dx + x) as usize, (dy + y) as usize))
            {
                neighbours.push(((dx + x) as usize, (dy + y) as usize));
            }
        }
    }

    neighbours
}

fn shortest_path(
    graph: &Graph,
    start: (usize, usize),
    end: (usize, usize),
    target: (usize, usize),
) -> usize {
    // bfs again
    // "end" is the "end of the path" node.
    // "target" is the node which contains disk which we want to transfer to (0, 0)-node.
    // bfs must go around "target" node in a path, because we want to keep an eye on "target"'s content.

    let mut visited = HashSet::new();
    let mut processing = VecDeque::new();
    let mut min_dist = usize::MAX;

    processing.push_back((start.0, start.1, 0));
    visited.insert(start);

    while !processing.is_empty() {
        let current_node = processing.pop_front().expect("not empty queue");
        let current_node_size = graph
            .get(&(current_node.0, current_node.1))
            .expect("valid node");

        let dist = current_node.2;

        if (current_node.0, current_node.1) == end {
            min_dist = min(dist, min_dist);
        }

        for neighbour in get_neighbours(graph, &(current_node.0, current_node.1)) {
            if visited.contains(&neighbour) || (neighbour == target) {
                // already visited or "target" node.
                continue;
            }

            let neighbour_size = graph.get(&neighbour).expect("valid neighbour");

            if neighbour_size.1 <= current_node_size.0 {
                // println!("{:?} {:?}", current_node, current_node_size);
                // println!("    {:?} {:?}", neighbour, neighbour_size);
                visited.insert(neighbour);
                processing.push_back((neighbour.0, neighbour.1, dist + 1));
            }
        }
    }

    min_dist
}

fn read_input(filename: &str) -> Result<Vec<Node>, Box<dyn error::Error>> {
    let nodes = fs::read_to_string(filename)?
        .lines()
        .skip(2)
        .map(|line| Node::from(line))
        .collect();

    Ok(nodes)
}

fn is_neighbours(a: &(usize, usize), b: &(usize, usize)) -> bool {
    let diff_x = if a.0 > b.0 { a.0 - b.0 } else { b.0 - a.0 };
    let diff_y = if a.1 > b.1 { a.1 - b.1 } else { b.1 - a.1 };
    (diff_x == 1 && diff_y == 0) || (diff_y == 1 && diff_x == 0)
}

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("--- Day 22: Grid Computing ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no input file as cli argument is provided")?;

    println!("{input_file}");

    let input_data = read_input(&input_file)?;
    let graph = build_graph(&input_data);
    let mut viable_count = 0;

    let mut start_points = HashSet::new();
    let mut max_x = 0;

    for (node_a, size_a) in &graph {
        max_x = max(max_x, node_a.0);
        for (node_b, size_b) in &graph {
            if node_a != node_b && size_a.1 != 0 && size_a.1 <= size_b.0 - size_b.1 {
                if is_neighbours(node_a, node_b) {
                    start_points.insert(node_b);
                    // println!("{:?} {:?} {:?} {:?}", node_a, size_a, node_b, size_b);
                }
                viable_count += 1;
            }
        }
    }
    println!("{viable_count}");

    let mut min_steps = usize::MAX;

    // println!("{max_x}");

    for start_point in start_points {
        let mut steps = 0;
        let mut current_start = *start_point;
        let mut current_target_x = max_x;

        while current_target_x >= 1 {
            let current_end = (current_target_x - 1, 0);
            let current_target = (current_target_x, 0);
            // stepping near the target (from the left),
            // without going through the target.
            steps += shortest_path(&graph, current_start, current_end, current_target);
            // 1 step to reach the target
            steps += 1;
            current_target_x = current_target_x - 1;
            current_start = (current_target_x + 1, 0);
        }
        min_steps = min(steps, min_steps);
    }

    println!("{min_steps}");

    Ok(())
}
