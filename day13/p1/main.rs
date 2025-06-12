#[rustfmt::skip]
fn is_wall(x: u32, y: u32) -> bool {
    // designers favourite number
    let dfn = 1358;
    let n = x*x + 3*x + 2*x*y + y + y*y;

    (n + dfn).count_ones() % 2 == 0
}

fn main() {
    println!("--- Day 13: A Maze of Twisty Little Cibicles ---");
}
