fn main() {
    println!("--- Day 19: An Elephant Named Joseph ---");

    let elf_count = 3018458;
    println!("{}", winner(elf_count));
}

fn winner(n: u32) -> u32 {
    1 + (n - u32::pow(2, (n.ilog2() as f32).floor() as u32)) * 2
}
