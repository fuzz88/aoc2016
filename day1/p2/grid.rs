// to have distance in taxicab geometry https://en.wikipedia.org/wiki/Taxicab_geometry
// we need to use Manhattan distance. lets figure out coordinates of the the end,
// then calculate our Manhattan distance from (0, 0)

// lets use Point for point coordinates
// will use this in HashSet to find already visited point
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new() -> Self {
        Point { x: 0, y: 0 }
    }
}

#[derive(Debug)]
pub enum Dir {
    L,
    R,
}

#[derive(Debug)]
pub struct Instr {
    dir: Dir,
    length: u32,
}

// use this as iterator for all the points we visiting following instructions
pub struct EasterBunnyRecruitingDocument {
    path: Vec<Instr>,
    index: usize,
    azimut: i32, //angle, clockwise,  0 - North, 90 - East, 180 - South, 270 - West,
    stand_at: Point,
    new_instr: bool,
}

impl EasterBunnyRecruitingDocument {
    pub fn new(path: &str) -> Self {
        // parsing path to list of instructions
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

        Self {
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
        // if this the last instruction -- stop iteration
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
        // move one atomic step towards the given azimut
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
        // checks current instruction is fully exhausted
        if curr_instr.length == 0 {
            // then prepare next instruction
            self.new_instr = true;
            self.index += 1;
        }
        Some(self.stand_at.clone())
    }
}

pub fn manhattan_distance(point: Point) -> u32 {
    point.x.abs() as u32 + point.y.abs() as u32
}
