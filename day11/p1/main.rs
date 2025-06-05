use std::env;

fn main() -> Result<(), &'static str> {
    println!("--- Day 11: Radioisotope Thermoelectric Generators ---");

    let input_file = env::args()
        .nth(1)
        .ok_or("no input file provided")?;

    println!("{}", input_file);



    Ok(())
}
