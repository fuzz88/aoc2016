use std::env;
use std::fmt;
use std::fs;
use std::io;
use std::sync::{OnceLock, RwLock};

static ELEMENTS: OnceLock<RwLock<Vec<String>>> = OnceLock::new();

fn get_elements() -> &'static RwLock<Vec<String>> {
    ELEMENTS.get_or_init(|| RwLock::new(vec![]))
}

#[derive(Debug)]
enum ItemType {
    Microchip,
    Generator,
}

struct Item {
    item_type: ItemType,
    element_id: usize,
    floor: u32,
}

impl fmt::Debug for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let elements = get_elements().read().unwrap();
        let name = &elements[self.element_id];

        f.debug_struct("Item")
            .field("item_type", &self.item_type)
            .field("element_id", &self.element_id)
            .field("(debug) elements_name", name)
            .field("floor", &self.floor)
            .finish()
    }
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
    let items = fs::read_to_string(filename)?
        .lines()
        .map(|line| parse_line(line))
        .flatten()
        .collect();

    Ok(items)
}

fn element_id(name: &str) -> usize {
    let mut elements = get_elements().write().unwrap();
    if let Some(idx) = elements.iter().position(|el| el == name) {
        return idx;
    }
    elements.push(name.to_string());
    elements.len() - 1
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
            element_id: element_id(element_name),
            floor,
        }),
        [element_name, "microchip," | "microchip." | "microchip"] => items.push(Item {
            item_type: ItemType::Microchip,
            element_id: element_id(
                element_name
                    .split("-")
                    .nth(0)
                    .expect("expecting dash in the type name of microchip"),
            ),
            floor,
        }),
        _ => {}
    });

    items
}
