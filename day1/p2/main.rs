// to have distance in taxicab geometry https://en.wikipedia.org/wiki/Taxicab_geometry
// we need to use Manhattan distance. lets figure out coordinates of the the end,
// then calculate our Manhattan distance from (0, 0)

use std::collections::HashSet;
use std::fs;

// lets use Point for point coordinates
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
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
    new_instr: bool,
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
            new_instr: true,
        }
    }
}

impl Iterator for EasterBunnyRecruitingDocument {
    type Item = Point;

    // let this give us all points in between
    fn next(&mut self) -> Option<Self::Item> {
        // if this last instruction -- stop iteration
        if self.index == self.path.len() {
            return None;
        }
        let curr_instr = &mut self.path[self.index];

        // exhausing current instruction or this is a new?
        if self.new_instr {
            // if new -- lets correct the azimut
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
        }
        // move one atomic step towards the give azimut
        match self.azimut {
            0 => {
                self.stand_at.y -= 1 as i32;
            }
            90 => {
                self.stand_at.x += 1 as i32;
            }
            180 => {
                self.stand_at.y += 1 as i32;
            }
            270 => {
                self.stand_at.x -= 1 as i32;
            }
            _ => unreachable!(),
        }
        // same azimut, smaller length
        curr_instr.length -= 1;
        self.new_instr = false;
        // checks if current instruction is fully exhausted
        if curr_instr.length == 0 {
            // if fully exhausted then prepare next instruction
            self.new_instr = true;
            self.index += 1;
        }
        Some(self.stand_at.clone())
    }
}

fn manhattan_distance(point: Point) -> u32 {
    point.x.abs() as u32 + point.y.abs() as u32
}

fn main() {
    //reading input
    //we will have all the examples in input.txt too
    let input: String = fs::read_to_string("input.txt")
        .expect("ERROR: failed to read inpuit.txt")
        .to_string();

    for path in input.lines() {
        let mut points = HashSet::new();
        let ebrd = EasterBunnyRecruitingDocument::new(path);
        for next_point in ebrd {
            if points.contains(&next_point) {
                println!("dist = {}", manhattan_distance(next_point));
                break;
            }
            points.insert(next_point);
        }
    }
}
