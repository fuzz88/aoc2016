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

    fn shift_row(&mut self, index: usize, count: usize) {
        for _ in 0..count {
            let mut tmp_pixels: Vec<char> = vec![];
            for idx in 0..self.width {
                tmp_pixels.push(self.pixels[index * self.width + idx]);
            }

            for idx in 0..self.width {
                let idx2 = (idx + 1) % self.width;
                self.pixels[index * self.width + idx2] = tmp_pixels[idx];
            }
        }
    }

    fn shift_column(&mut self, index: usize, count: usize) {
        for _ in 0..count {
            let mut tmp_pixels: Vec<char> = vec![];
            for shift in 0..self.height {
                tmp_pixels.push(self.pixels[shift * self.width + index]);
            }

            for shift in 0..self.height {
                let shift2 = (shift + 1) % self.height;
                self.pixels[shift2 * self.width + index] = tmp_pixels[shift];
            }
        }
    }

    fn rect(&mut self, width: usize, height: usize) {
        for shift in 0..height {
            for idx in 0..width {
                self.pixels[shift * self.width + idx] = '#';
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
            Command::Rect { width, height } => {
                screen.rect(*width, *height);
            }
            Command::Rotate {
                direction,
                index,
                count,
            } => match direction {
                Direction::Column => {
                    screen.shift_column(*index, *count);
                }
                Direction::Row => {
                    screen.shift_row(*index, *count);
                }
            },
        }
    }
}
