use std::fs;
use std::collections::HashSet;

mod grid;

fn main() {
    //reading input
    //we will have all the examples in input.txt too
    let input: String = fs::read_to_string("input.txt")
        .expect("ERROR: failed to read inpuit.txt")
        .to_string();

    for (num, path) in input.lines().enumerate() {
        let mut visited_points = HashSet::new();
        let all_the_points_through_the_path = grid::EasterBunnyRecruitingDocument::new(path);
        for next_point in all_the_points_through_the_path {
            if visited_points.contains(&next_point) {
                // first already visited
                println!("{num}: dist = {}", grid::manhattan_distance(next_point));
                break;
            }
            visited_points.insert(next_point);
        }
    }
}
