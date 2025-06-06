use std::env;
use std::fs;

type Result<T> = std::result::Result<T, String>;

#[derive(Debug, Clone)]
enum Item {
    Microchip { element: String, floor: u32 },
    Generator { element: String, floor: u32 },
}

fn main() -> Result<()> {
    println!("--- Day 11: Radioisotope Thermoelectric Generators ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no input filename as cli argument")?;

    let input_data = read_input(&input_file);

    println!("{:#?}", input_data);

    Ok(())
}

fn read_input(filename: &str) -> Result<Vec<Item>> {
    let items = fs::read_to_string(filename)
        .map_err(|err| format!("`{filename}`: {err}"))?
        .lines()
        .map(|line| parse_line(line))
        .collect::<Vec<Vec<Item>>>()
        .into_iter()
        .flatten()
        .collect();

    Ok(items)
}

fn parse_line(line: &str) -> Vec<Item> {
    let mut items: Vec<Item> = vec![];
    let components: Vec<&str> = line.split_whitespace().collect();

    // always starting from the first floor
    let mut floor: u32 = 1;
    components.windows(2).for_each(|pair| match pair {
        ["The", "second"] => floor = 2,
        ["The", "third"] => floor = 3,
        ["The", "fourth"] => floor = 4,
        [element, "generator," | "generator." | "generator"] => items.push(Item::Generator {
            element: element.to_string(),
            floor,
        }),
        [element, "microchip," | "microchip." | "microchip"] => items.push(Item::Microchip {
            element: element.split("-").nth(0).unwrap().to_string(),
            floor,
        }),
        _ => {}
    });

    items
}
