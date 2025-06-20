use std::cmp::Ordering;
use std::env;
use std::error;
use std::fs;

#[derive(Debug, Clone)]
struct IPRange {
    low: u32,
    high: u32,
}

impl IPRange {
    fn count(&self) -> u32 {
        self.high - self.low + 1
    }
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
    println!("lhs: {:#?}", lhs);
    println!("rhs: {:#?}", rhs);
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
        (Ordering::Equal, _, _, Ordering::Greater) => {
            result.push(IPRange {
                low: rhs.high + 1,
                high: lhs.high,
            });
        }
        (_, _, Ordering::Greater, _) => {
            result.push(lhs.clone());
        }
        (_, Ordering::Equal, _, _) => {
            result.push(IPRange {
                low: lhs.low,
                high: lhs.high - 1,
            });
        }
        (_, Ordering::Less, _, _) => {
            result.push(lhs.clone());
        }
        (Ordering::Less, Ordering::Greater, _, Ordering::Less) => {
            result.push(IPRange {
                low: lhs.low,
                high: rhs.low - 1,
            });
        }
        (Ordering::Less, Ordering::Greater, _, Ordering::Equal) => {
            result.push(IPRange {
                low: lhs.low,
                high: rhs.low - 1,
            });
        }
        (Ordering::Equal, Ordering::Greater, Ordering::Less, Ordering::Less) => {}
        (Ordering::Equal, _, _, Ordering::Equal) => {}
        (Ordering::Greater, Ordering::Greater, Ordering::Less, Ordering::Greater) => {
            result.push(IPRange {
                low: rhs.high + 1,
                high: lhs.high,
            });
        }
        (
            Ordering::Greater,
            Ordering::Greater,
            Ordering::Less,
            Ordering::Less | Ordering::Equal,
        ) => {}

        value => todo!("{:?}", value),
    }

    result
}

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("--- Day 20: Firewall Rules ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no input file as cli argument is provided")?;

    let input_data = read_input(&input_file)?;

    // println!("{input_data:#?}");

    let mut next_iter = sub_ranges(&input_data[0], &input_data[1]);
    for disallowed in &input_data[2..] {
        let mut iter_ranges: Vec<IPRange> = vec![];
        for allowed in &next_iter {
            iter_ranges.extend(sub_ranges(&allowed, &disallowed));
        }
        next_iter = iter_ranges;
    }

    next_iter.sort_by_key(|iprange| iprange.low);

    println!("{}", next_iter[0].low);

    println!(
        "{}",
        next_iter.iter().map(|iprange| iprange.count()).sum::<u32>()
    );

    Ok(())
}
