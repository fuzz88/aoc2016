use std::cmp::Ordering;
use std::env;
use std::error;
use std::fs;

#[derive(Debug)]
struct IPRange {
    low: u32,
    high: u32,
}

fn read_input(filename: &str) -> Result<Vec<IPRange>, Box<dyn error::Error>> {
    let ranges = fs::read_to_string(filename)?
        .lines()
        .map(|line| parse_line(line))
        .collect();

    Ok(ranges)
}

fn parse_line(line: &str) -> IPRange {
    let components: Vec<&str> = line.split('-').collect();

    let low = components[0].parse().unwrap();
    let high = components[1].parse().unwrap();

    IPRange { low, high }
}
/// lhs is what allowed, rhs is what NOT allowed.
fn sub_ranges(lhs: &IPRange, rhs: &IPRange) -> Vec<IPRange> {
    let mut result = Vec::new();
    match (
        lhs.low.cmp(&rhs.low),
        lhs.high.cmp(&rhs.low),
        lhs.low.cmp(&rhs.high),
        lhs.high.cmp(&rhs.high),
    ) {
        (Ordering::Less, _, _, Ordering::Greater) => {
            result.push(IPRange {
                low: lhs.low,
                high: rhs.low - 1,
            });
            result.push(IPRange {
                low: rhs.high + 1,
                high: lhs.high,
            });
        }
        _ => todo!(),
    }

    result
}

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("--- Day 20: Firewall Rules ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no input file as cli argument is provided")?;

    let input_data = read_input(&input_file)?;

    println!("{input_data:#?}");

    println!("{:#?}", sub_ranges(&input_data[0], &input_data[1]));

    Ok(())
}
