use std::env;
use std::fs;
use std::io;

#[derive(Debug, Clone)]
enum Item {
    Microchip { element: String, floor: u32 },
    Generator { element: String, floor: u32 },
}

fn main() -> Result<(), io::Error> {
    println!("--- Day 11: Radioisotope Thermoelectric Generators ---");

    let input_file = env::args().nth(1).ok_or(io::Error::new(
        io::ErrorKind::InvalidInput,
        "no input filename as cli argument",
    ))?;

    let input_data = read_input(&input_file)?;

    println!("{:#?}", input_data);

    Ok(())
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

    // always starting from the first floor
    let mut floor: u32 = 1;
    // parse floor information by iterating pairs of tokens.
    components.windows(2).for_each(|pair| match pair {
        ["The", "second"] => floor = 2,
        ["The", "third"] => floor = 3,
        ["The", "fourth"] => floor = 4,
        [element, "generator," | "generator." | "generator"] => items.push(Item::Generator {
            element: element.to_string(),
            floor,
        }),
        [element, "microchip," | "microchip." | "microchip"] => items.push(Item::Microchip {
            element: element
                .split("-")
                .nth(0)
                .expect("expecting dash in the type name of microchip")
                .to_string(),
            floor,
        }),
        _ => {}
    });

    items
}
