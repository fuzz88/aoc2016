// to have distance in taxicab geometry https://en.wikipedia.org/wiki/Taxicab_geometry
// we need to use Manhattan distance. lets figure out coordinates of the the end,
// then calculate our Manhattan distance from (0, 0)

use std::fs;

// lets use Point for point coordinates
#[derive(Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new() -> Point {
        Point { x: 0, y: 0 }
    }
}

#[derive(Debug)]
enum Dir {
    L,
    R,
}
#[derive(Debug)]
struct Instr {
    dir: Dir,
    length: u32,
}

struct EasterBunnyRecruitingDocument {
    path: Vec<Instr>,
    index: usize,
    azimut: i32, //angle, 0 - North, 180 - South, 90 - West, 270 - East
    stand_at: Point,
}

impl EasterBunnyRecruitingDocument {
    fn new(path: &str) -> EasterBunnyRecruitingDocument {
        let path: Vec<Instr> = path
            .split(" ")
            .map(|instruction| {
                let (d, l) = instruction.trim_end_matches(",").split_at(1);
                Instr {
                    dir: match d {
                        "R" => Dir::R,
                        "L" => Dir::L,
                        _ => unreachable!(),
                    },
                    length: l.parse().unwrap(),
                }
            })
            .collect();
        EasterBunnyRecruitingDocument {
            path,
            index: 0,
            azimut: 0,
            stand_at: Point::new(),
        }
    }
}

impl Iterator for EasterBunnyRecruitingDocument {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.path.len() {
            return None;
        }
        let curr_instr = &self.path[self.index];
        self.azimut += match curr_instr.dir {
            Dir::R => 90,
            Dir::L => -90,
        };
        if self.azimut < 0 {
            self.azimut += 360;
        }
        if self.azimut >= 360 {
            self.azimut -= 360;
        }
        match self.azimut {
            0 => {
                self.stand_at.y -= curr_instr.length as i32;
            }
            90 => {
                self.stand_at.x += curr_instr.length as i32;
            }
            180 => {
                self.stand_at.y += curr_instr.length as i32;
            }
            270 => {
                self.stand_at.x -= curr_instr.length as i32;
            }
            _ => unreachable!(),
        }
        self.index += 1;
        Some(self.stand_at.clone())
    }
}

fn manhattan_distance(point: Point) -> u32 {
    point.x.abs() as u32 + point.y.abs() as u32
}

fn find_first_visited(points: Vec<Point>) -> Point {
    Point::new()
}

fn main() {
    //reading input
    //we will have all the examples in input.txt too
    let input: String = fs::read_to_string("input.txt")
        .expect("ERROR: failed to read inpuit.txt")
        .to_string();

    for path in input.lines() {
        let mut points: Vec<Point> = vec![];
        let mut ebrd = EasterBunnyRecruitingDocument::new(path);
        for next_point in ebrd {
            points.push(next_point);
        }
        dbg!(points);
    }
}
