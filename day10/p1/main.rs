fn main() {
    println!("--- Day 10: Balance Bots ---");

    let input_file = std::env::args()
        .nth(1)
        .expect("expecting input filename as an argument");

    let (bots, rules) = read_input(&input_file);
    println!("{}", input_file);
    println!("{:?} {:?}", bots, rules);
}

type Output = Vec<u32>; // Output is an array of microchips.

type Bot = [u32; 2]; // Bot is basically described as a two microchips it holds.
type Bots = Vec<Bot>;

type Rule = ();
type Rules = Vec<Rule>;

fn read_input(filename: &str) -> (Bots, Rules) {
    let mut bots: Bots = vec![];
    let mut rules: Rules = vec![];

    std::fs::read_to_string(filename)
        .expect("expecting valid string in input file")
        .lines()
        .for_each(|line| {
            if line.starts_with("v") {
                // "value N goes to ..."
                bots.push(parse_bot(line));
            } else {
                // "bot 0 gives ..."
                rules.push(parse_rule(line));
            }
        });

    (bots, rules)
}

fn parse_bot(line: &str) -> Bot {
    [1, 2]
}

fn parse_rule(line: &str) -> Rule {
    ()
}
