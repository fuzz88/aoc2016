// to have distance in taxicab geometry https://en.wikipedia.org/wiki/Taxicab_geometry
// we need to use Manhattan distance. lets figure out coordinates of the the end,
// then calculate our Manhattan distance from (0, 0)

use std::fs;

fn main () {
    //reading input
    //we will have all the examples in input.txt too
    let input: String = fs::read_to_string("input.txt")
        .expect("ERROR: failed to read inpuit.txt")
        .to_string();
    
    for path in input.lines() {
        let mut azimut: i32 = 0; //angle, 0 - North, 180 - South, 90 - West, 270 - East
        let mut end: (i32, i32) = (0, 0);
        //e.g. "R1, L18"
        for instruction in path.split(" ") {
            let instruction = instruction.trim_end_matches(",");
            // direction, length
            let (d, l) = instruction.split_at(1);
            let l: i32 = l.parse().unwrap();
            azimut += match d {
                "R" => -90,
                "L" => 90,
                _ => unreachable!(),
            };
            if azimut < 0 { azimut += 360; };
            if azimut >= 360 { azimut -= 360; };
            match azimut {
                0 => { end.1 += l; },
                90 => { end.0 -= l; },
                180 => { end.1 -= l; },
                270 => { end.0 += l; },
                _ => unreachable!(),
            };
        }
        println!("dist = {}", end.0.abs() + (end.1).abs())
    }
}
