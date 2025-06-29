use std::env;
use std::fmt;
use std::fs;
use std::io;

#[derive(Debug)]
enum ItemType {
    Microchip,
    Generator,
}

#[derive(Debug)]
struct Item {
    item_type: ItemType,
    element: String,
    floor: u32,
}

fn main() -> Result<(), io::Error> {
    println!("--- Day 11: Radioisotope Thermoelectric Generators ---");

    let input_file = env::args().nth(1).ok_or(io::Error::new(
        io::ErrorKind::InvalidInput,
        "no input filename as cli argument",
    ))?;

    let mut input_data = read_input(&input_file)?;
    println!("{:#?}", input_data);

    elevator(&mut input_data, [Some(0), Some(1)], 2);

    println!("{:#?}", input_data);

    // println!("{}", solver(&input_data));

    Ok(())
}

fn solver(items: &Vec<Item>) -> u32 {
    let mut solutions_step_count: Vec<u32> = vec![];

    solutions_step_count.push(99);
    //
    // some way of searching.
    // some combinatorics.
    //
    // how many operations are needed to move all items, if there are no limitations?
    //
    // limitations:
    //
    //      1. both generator and microchip are blockers, and this adds operations.
    //      2. elevator must not be empty, and this adds operations.
    //
    //
    // intuition tells me that there is kind of formula:
    //
    // min_ops = ops_without_limitations + ops_to_keep_invariants
    //
    // and ops_to_keep_invariants depends of relative positions of blockers and elevator mechanics.
    //
    // but what if we want actual sequence of operations?
    // how to implement full enumeration of operational sequences which are leading to desired
    // results? how to implement branch pruning and backtracking of variants?
    //

    *solutions_step_count
        .iter()
        .max()
        .expect("expecting at least one solution to be found")
}

fn elevator(items: &mut Vec<Item>, idxs: [Option<usize>; 2], end_floor: u32) -> bool { 
    // takes two or one item and tries to move it to target floor
    if let Some(id1) = idxs[0] {
        let item1 = items.get_mut(id1).unwrap();
        item1.floor = end_floor;
    }
    if let Some(id2) = idxs[1] {
        let item2 = items.get_mut(id2).unwrap();
        item2.floor = end_floor;
    }
    true
}

// fn floor_items(items: &Vec<Item>, floor: u32) -> Vec<Item> {
//     items
//         .into_iter()
//         .filter(|item| item.floor == floor)
//         .collect()
// }

fn read_input(filename: &str) -> Result<Vec<Item>, io::Error> {
    let items = fs::read_to_string(filename)?
        .lines()
        .map(|line| parse_line(line))
        .flatten()
        .collect();

    Ok(items)
}

fn parse_line(line: &str) -> Vec<Item> {
    let mut items: Vec<Item> = vec![];
    let components: Vec<&str> = line.split_whitespace().collect();

    let mut floor: u32;
    match components[1] {
        "first" => floor = 1,
        "second" => floor = 2,
        "third" => floor = 3,
        "fourth" => floor = 4,
        _ => unreachable!(),
    }
    // parse floor information by iterating pairs of tokens.
    components[2..].windows(2).for_each(|pair| match pair {
        [element_name, "generator," | "generator." | "generator"] => items.push(Item {
            item_type: ItemType::Generator,
            element: element_name.to_string(),
            floor,
        }),
        [element_name, "microchip," | "microchip." | "microchip"] => items.push(Item {
            item_type: ItemType::Microchip,
            element: element_name
                .split("-")
                .nth(0)
                .expect("expecting dash in the type name of microchip").to_string(),
            floor,
        }),
        _ => {}
    });

    items
}
