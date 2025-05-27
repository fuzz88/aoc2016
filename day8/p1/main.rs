fn main() {
    println!("--- Day8: Two-Factor Authentication ---");

    let input_file = std::env::args()
        .nth(1)
        .expect("expecting input file as command line argument");

    let commands = read_input(&input_file);

    println!("{:#?}", &commands);
}

fn read_input(filename: &str) -> Vec<Command> {
    std::fs::read_to_string(filename)
        .expect("expecting valid utf-8 text file")
        .lines()
        .map(|line| parse_command(line))
        .collect()
}

fn parse_command(line: &str) -> Command {
    let components: Vec<&str> = line.split_whitespace().collect();

    if components.len() == 2 {
        assert!(components[0] == "rect");

        let params: Vec<&str> = components[1].split("x").collect();
        let width: u32 = params[0].parse().unwrap();
        let height: u32 = params[1].parse().unwrap();

        return Command::Rect { width, height };
    }

    if components.len() == 5 {
        assert!(components[0] == "rotate");

        let direction = match components[1] {
            "column" => Direction::Column,
            "row" => Direction::Row,
            _ => unreachable!(),
        };
        let index = components[2].split("=").nth(1).unwrap().parse().unwrap();
        let count = components[4].parse().unwrap();

        return Command::Rotate {
            direction,
            index,
            count,
        };
    }
    unreachable!();
}

#[derive(Debug)]
enum Direction {
    Column,
    Row,
}

#[derive(Debug)]
enum Command {
    Rect {
        width: u32,
        height: u32,
    },
    Rotate {
        direction: Direction,
        index: u32,
        count: u32,
    },
}
