use std::collections::HashMap;
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

fn get_neighbours(x_y: &(usize, usize), graph: &Graph) -> Vec<(usize, usize)> {
    let mut neightbours = vec![];
    let x = x_y.0 as i32;
    let y = x_y.1 as i32;

    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx != 0 && dy != 0 {
                if x + dx >= 0 && y + dy >= 0 {
                    let x = x as usize;
                    let y = y as usize;
                    if let Some(_) = graph.get(&(x, y)) {
                        neightbours.push((x, y));
                    }
                }
            }
        }
    }

    neightbours
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

    println!("{graph:?}");

    Ok(())
}
