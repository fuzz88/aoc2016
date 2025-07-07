use std::collections::{HashSet, VecDeque};
use std::env;
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
    // println!("{:#?}", input_data);

    let field1 = items_to_vec(&input_data);
    // println!("{:#?}", field);
    println!("{}", solver(&field1));

    input_data.push(Item {
        item_type: ItemType::Generator,
        element: "elerium".to_string(),
        floor: 1,
    });
    input_data.push(Item {
        item_type: ItemType::Microchip,
        element: "elerium".to_string(),
        floor: 1,
    });
    input_data.push(Item {
        item_type: ItemType::Generator,
        element: "dolithium".to_string(),
        floor: 1,
    });
    input_data.push(Item {
        item_type: ItemType::Microchip,
        element: "dilithium".to_string(),
        floor: 1,
    });

    let field2 = items_to_vec(&input_data);
    println!("{}", solver(&field2));

    Ok(())
}

fn is_bad_state(state: &Vec<u32>) -> bool {
    for i in 0..state.len() {
        if i % 2 == 0 {
            for j in 0..state.len() {
                if j % 2 != 0 {
                    if (i / 2) != (j / 2) {
                        // println!("{} {}", state[i], state[j]);
                        // println!("{} {}", i, j);
                        if state[i] == state[j] && state[i] != state[i + 1] {
                            // println!("bad {} {}", i, j);
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}

fn solver(state: &Vec<u32>) -> u32 {
    let mut min_steps = u32::MAX;

    let mut seen = HashSet::<Vec<u32>>::new();
    let mut to_process = VecDeque::<Vec<u32>>::new();

    let mut state = state.clone();
    state.push(0);

    seen.insert(state.clone());
    to_process.push_back(state.clone());

    while let Some(mut current_state) = to_process.pop_front() {
        let steps = current_state.pop().unwrap();

        if seen.contains(&current_state) {
            continue;
        } else {
            seen.insert(current_state.clone());
        }

        let elevator = current_state.pop().unwrap();

        if elevator < 1 || elevator > 4 {
            continue;
        }

        if current_state.iter().all(|item| *item == 4) {
            if steps < min_steps {
                min_steps = steps;
            }
            // println!("steps: {}", steps);
            break;
        }

        let mut can_move = vec![];

        if !is_bad_state(&current_state) {
            // println!("{:?}", current_state);
            for idx in 0..current_state.len() {
                if current_state[idx] == elevator {
                    can_move.push(idx);
                }
            }
            // println!("{:#?}", can_move);

            for i in 0..can_move.len() {
                let mut state = current_state.clone();
                state[can_move[i]] += 1;
                state.push(elevator + 1);
                state.push(steps + 1);
                // println!("{:?}", state);
                to_process.push_back(state);
            }
            for i in 0..can_move.len() {
                let mut state = current_state.clone();
                state[can_move[i]] -= 1;
                state.push(elevator - 1);
                state.push(steps + 1);
                // println!("{:?}", state);
                to_process.push_back(state);
            }

            for i in 0..can_move.len() {
                for j in 0..can_move.len() {
                    if i != j {
                        let mut state = current_state.clone();
                        state[can_move[i]] += 1;
                        state[can_move[j]] += 1;
                        state.push(elevator + 1);
                        state.push(steps + 1);
                        // println!("{:?}", state);
                        to_process.push_back(state);
                    }
                }
            }

            for i in 0..can_move.len() {
                for j in 0..can_move.len() {
                    if i != j {
                        let mut state = current_state.clone();
                        state[can_move[i]] -= 1;
                        state[can_move[j]] -= 1;
                        state.push(elevator - 1);
                        state.push(steps + 1);
                        // println!("{:?}", state);
                        to_process.push_back(state);
                    }
                }
            }
            // println!("--------------------------------------");
        }
    }

    min_steps
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

    results.push(1); // elevator
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

    let floor: u32;
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
                .expect("expecting dash in the type name of microchip")
                .to_string(),
            floor,
        }),
        _ => {}
    });

    items
}
