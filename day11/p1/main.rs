use std::env;
use std::fmt;
use std::fs;
use std::io;

#[derive(Debug, Clone, Ord, Eq, PartialEq, PartialOrd)]
enum ItemType {
    Generator,
    Microchip,
}

#[derive(Debug, Clone)]
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

    println!("{:#?}", items_to_vec(&input_data));

    // println!("{}", solver(&input_data));

    Ok(())
}

fn items_to_vec(items: &Vec<Item>) -> Vec<u32> {
    let mut results = vec![];
    let mut items = items.clone();
    items.sort_by_key(|item| item.item_type.clone());
    items.sort_by_key(|item| item.element.clone());
    loop {
        if items.is_empty() {
            break;
        }

        let item1 = items.pop().unwrap();
        let item2 = items.pop().unwrap();

        results.push(item1.floor);
        results.push(item2.floor);
    }

    results
}

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
