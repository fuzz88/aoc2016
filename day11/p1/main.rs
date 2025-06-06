use std::env;
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
    element_id: usize,
    floor: u32,
}

fn main() -> Result<(), io::Error> {
    println!("--- Day 11: Radioisotope Thermoelectric Generators ---");

    let input_file = env::args().nth(1).ok_or(io::Error::new(
        io::ErrorKind::InvalidInput,
        "no input filename as cli argument",
    ))?;

    let input_data = read_input(&input_file)?;
    println!("{:#?}", input_data);

    println!("{}", solver(&input_data));

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

fn take_elevator(items: Vec<Item>, idxs_to_move: [usize; 2], end_floor: u32) -> bool {
    true
}

fn floor_items(items: Vec<Item>, floor: u32) -> Vec<Item> {
    items
        .into_iter()
        .filter(|item| item.floor == floor)
        .collect()
}

fn read_input(filename: &str) -> Result<Vec<Item>, io::Error> {
    // names of elements will be internalized in this vector.
    // then it will be dropped, because we don't actially need this names.
    // unique indexes as ids is just enough.
    let mut elements: Vec<String> = vec![];

    let items = fs::read_to_string(filename)?
        .lines()
        .map(|line| parse_line(line, &mut elements))
        .flatten()
        .collect();

    Ok(items)
}

fn element_id(name: &str, elements: &mut Vec<String>) -> usize {
    if let Some(idx) = elements.iter().position(|el| el == name) {
        return idx;
    }
    elements.push(name.to_string());
    elements.len() - 1
}

fn parse_line(line: &str, elements: &mut Vec<String>) -> Vec<Item> {
    let mut items: Vec<Item> = vec![];
    let components: Vec<&str> = line.split_whitespace().collect();

    // always starting from the first floor.
    let mut floor: u32 = 1;
    // parse floor information by iterating pairs of tokens.
    components.windows(2).for_each(|pair| match pair {
        ["The", "second"] => floor = 2,
        ["The", "third"] => floor = 3,
        ["The", "fourth"] => floor = 4,
        [element_name, "generator," | "generator." | "generator"] => items.push(Item {
            item_type: ItemType::Generator,
            element_id: element_id(element_name, elements),
            floor,
        }),
        [element_name, "microchip," | "microchip." | "microchip"] => items.push(Item {
            item_type: ItemType::Microchip,
            element_id: element_id(
                element_name
                    .split("-")
                    .nth(0)
                    .expect("expecting dash in the type name of microchip"),
                elements,
            ),
            floor,
        }),
        _ => {}
    });

    items
}
