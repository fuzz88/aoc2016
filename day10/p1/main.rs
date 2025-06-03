use std::collections::HashMap;

fn main() {
    println!("--- Day 10: Balance Bots ---");

    let input_file = std::env::args()
        .nth(1)
        .expect("expecting input filename as an argument");

    let (bots, rules) = read_input(&input_file);
    println!("{}", input_file);
    println!("{:?}\n{:?}", bots, rules);
}

type Output = Vec<u32>; // "output" bin is an array of microchips.

type Bot = [Option<u32>; 2]; // Bot is basically described as a two microchips it holds.
type Bots = HashMap<u32, Bot>;

fn give_microchip(chip: u32, bot: &mut Bot) {
    if bot[0] == None {
        bot[0] = Some(chip);
        return;
    } else if bot[1] == None {
        bot[1] = Some(chip);
        return;
    }
    unreachable!("cant give to bot, it already has two microchips. wrong input");
}

fn take_higher(bot: &mut Bot) -> u32 {
    if bot[0] != None && bot[1] != None {
        if bot[0] < bot[1] {
            let higher = bot[1];
            bot[1] = None;
            return higher.unwrap();
        } else {
            let higher = bot[0];
            bot[0] = None;
            return higher.unwrap();
        }
    }
    unreachable!("cant take from bot with only one microchip. wrong input");
}

fn take_lower(bot: &mut Bot) -> u32 {
    if bot[0] != None && bot[1] != None {
        if bot[0] > bot[1] {
            let lower = bot[1];
            bot[1] = None;
            return lower.unwrap();
        } else {
            let lower = bot[0];
            bot[0] = None;
            return lower.unwrap();
        }
    }
    unreachable!("cant take from bot with only one microchip. wrong input");
}

type Rule = ();
type Rules = Vec<Rule>;

fn read_input(filename: &str) -> (Bots, Rules) {
    let mut bots: Bots = HashMap::new();
    let mut rules: Rules = vec![];

    std::fs::read_to_string(filename)
        .expect("expecting valid string in input file")
        .lines()
        .for_each(|line| {
            if line.starts_with("v") {
                // "value N goes to ..."
                let (value, bot_number) = parse_bot(line);
                if let Some(bot) = bots.get_mut(&bot_number) {
                    give_microchip(value, bot);
                } else {
                    let mut new_bot = [None; 2];
                    give_microchip(value, &mut new_bot);
                    bots.insert(bot_number, new_bot);
                }
            } else {
                // "bot N gives ..."
                rules.push(parse_rule(line));
            }
        });

    (bots, rules)
}

fn parse_bot(line: &str) -> (u32, u32) {
    //println!("bot: {}", line);
    let mut components = line.split_whitespace();
    (
        components
            .nth(1)
            .expect("expecting value at original position 1")
            .parse()
            .unwrap(),
        components
            .nth(3)
            .expect("expecting bot number at original position 5")
            .parse()
            .unwrap(),
    )
}

fn parse_rule(line: &str) -> Rule {
    println!("rule: {}", line);
    ()
}
