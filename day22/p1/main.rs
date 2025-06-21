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

    for dx in -1..=1 {
        for dy in -1..=1 {
            if !(dx == 0 && dy == 0) && (dx == 0 || dy == 0) {
                if x + dx >= 0 && y + dy >= 0 {
                    // println!("{} {}", dx, dy);
                    if let Some(_) = graph.get(&((dx + x) as usize, (dy + y) as usize)) {
                        neighbours.push(((dx + x) as usize, (dy + y) as usize));
                    }
                }
            }
        }
    }

    neighbours
}

fn walk_from_node(graph: &Graph, start: (usize, usize)) -> usize {
    let mut visited = HashSet::new();
    let mut processing = VecDeque::new();
    let mut viable_count = 0;

    processing.push_back(start);
    visited.insert(start);

    while !processing.is_empty() {
        let current_node = processing.pop_front().expect("not empty queue");
        let current_node_size = graph.get(&current_node).expect("valid node");

        for neighbour in get_neighbours(graph, &current_node) {
            if visited.contains(&neighbour) {
                continue;
            }

            let neighbour_size = graph.get(&neighbour).expect("valid neighbour");

            if neighbour_size.0 - neighbour_size.1 >= current_node_size.1
                && current_node_size.1 != 0
            {
                println!("{:?} {:?}", current_node, current_node_size);
                println!("    {:?} {:?}", neighbour, neighbour_size);
                visited.insert(neighbour);
                processing.push_back(neighbour);
                viable_count += 1;
            }
        }
    }

    println!("viable count = {}", viable_count);
    viable_count
}

fn read_input(filename: &str) -> Result<Vec<Node>, Box<dyn error::Error>> {
    let nodes = fs::read_to_string(filename)?
        .lines()
        .skip(2)
        .map(|line| Node::from(line))
        .collect();

    Ok(nodes)
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

    for (node_a, size_a) in &graph {
        for (node_b, size_b) in &graph {
            if node_a != node_b {
                if size_a.1 != 0 {
                    if size_a.1 <= size_b.0 - size_b.1 {
                        viable_count += 1;
                    }
                }
            }
        }
    }

    println!("{viable_count}");

    Ok(())
}
