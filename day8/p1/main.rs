fn main() {
    println!("--- Day8: Two-Factor Authentication ---");

    let input_file = std::env::args()
        .nth(1)
        .expect("expecting input file as command line argument");

    let commands = read_input(&input_file);
    let mut screen = Screen::new(50, 6);

    screen.process(&commands);

    println!();
    screen.print();

    println!("{}", screen.lit_count());
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
        let width = params[0].parse().unwrap();
        let height = params[1].parse().unwrap();

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

struct Screen {
    pixels: Vec<char>,
    width: usize,
    height: usize,
}

impl Screen {
    fn new(width: usize, height: usize) -> Screen {
        let pixels = vec!['.'; width * height];
        Screen {
            pixels,
            width,
            height,
        }
    }

    fn print(&self) {
        let mut idx = 0;
        loop {
            if idx == self.width * self.height {
                break;
            }
            if (idx + 1) % self.width == 0 {
                print!("{}\n", self.pixels[idx]);
            } else {
                print!("{} ", self.pixels[idx]);
            }
            idx += 1;
        }
        println!();
    }

    fn process(&mut self, commands: &Vec<Command>) {
        for command in commands {
            command.apply(self);
        }
    }

    fn lit_count(&self) -> u32 {
        self.pixels
            .iter()
            .map(|pixel| match pixel {
                '.' => 0,
                '#' => 1,
                _ => unreachable!(),
            })
            .sum()
    }

    fn shift_row(&mut self, row: usize, shift_count: usize) {
        for _ in 0..shift_count {
            let mut tmp_pixels: Vec<char> = vec![];
            for col in 0..self.width {
                let idx = row * self.width + col;

                tmp_pixels.push(self.pixels[idx]);
            }

            for col in 0..self.width {
                let next_col = (col + 1) % self.width;
                let idx = row * self.width + next_col;

                self.pixels[idx] = tmp_pixels[col];
            }
        }
    }

    fn shift_col(&mut self, col: usize, shift_count: usize) {
        for _ in 0..shift_count {
            let mut tmp_pixels: Vec<char> = vec![];
            for row in 0..self.height {
                let idx = row * self.width + col;

                tmp_pixels.push(self.pixels[idx]);
            }

            for row in 0..self.height {
                let next_row = (row + 1) % self.height;
                let idx = next_row * self.width + col;

                self.pixels[idx] = tmp_pixels[row];
            }
        }
    }

    fn rect(&mut self, width: usize, height: usize) {
        for row in 0..height {
            for col in 0..width {
                self.pixels[row * self.width + col] = '#';
            }
        }
    }
}

#[derive(Debug)]
enum Direction {
    Column,
    Row,
}

#[derive(Debug)]
enum Command {
    Rect {
        width: usize,
        height: usize,
    },
    Rotate {
        direction: Direction,
        index: usize,
        count: usize,
    },
}

impl Command {
    fn apply(&self, screen: &mut Screen) {
        match self {
            Command::Rotate {
                direction,
                index,
                count,
            } => match direction {
                Direction::Column => {
                    screen.shift_col(*index, *count);
                }
                Direction::Row => {
                    screen.shift_row(*index, *count);
                }
            },
            Command::Rect { width, height } => {
                screen.rect(*width, *height);
            }
        }
    }
}
