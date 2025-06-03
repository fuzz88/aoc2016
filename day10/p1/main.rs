#![feature(let_chains)]

use std::cmp::Ordering;
use std::collections::HashMap;

type Microchip = u32;
// actually we don't need outputs for now, but let's keep them in case.
type Output = HashMap<u32, Microchip>; // each "output" bin holds one microchip

// Bot is basically described as a two microchips it holds.
type Bot = [Option<Microchip>; 2];
type Bots = HashMap<u32, Bot>;

#[derive(Debug)]
enum Receiver {
    Bot(u32),
    Output(u32),
}
// Rule is the way to describe who receives lower and higher values.
type Rule = [Receiver; 2];
type Rules = HashMap<u32, Rule>;

fn main() {
    println!("--- Day 10: Balance Bots ---");

    let input_file = std::env::args()
        .nth(1)
        .expect("expecting input filename as an argument");

    let (bots, rules) = read_input(&input_file);
    println!("{}", input_file);
    println!("{:?}\n\n{:?}", bots, rules);
}

fn give(chip: Microchip, bot: &mut Bot) {
    for hand in bot.as_mut_slice() {
        if *hand == None {
            *hand = Some(chip);
            return;
        }
    }
    unreachable!("cant give to bot, it already has two microchips. wrong input");
}

fn take(bot: &mut Bot, this: Ordering) -> Microchip {
    if let Some(hand1) = bot[0]
        && let Some(hand2) = bot[1]
    {
        if hand1.cmp(&hand2) == this {
            return hand1;
        } else {
            return hand2;
        }
    }
    unreachable!("cant take from bot with only one microchip. wrong input");
}

fn read_input(filename: &str) -> (Bots, Rules) {
    let mut bots: Bots = HashMap::new();
    let mut rules: Rules = HashMap::new();

    std::fs::read_to_string(filename)
        .expect("expecting valid string in input file")
        .lines()
        .for_each(|line| {
            if line.starts_with("v") {
                // "value N goes to ..."
                let (microchip, bot_number) = parse_bot(line);
                if let Some(bot) = bots.get_mut(&bot_number) {
                    give(microchip, bot);
                } else {
                    let mut new_bot = [None; 2];
                    give(microchip, &mut new_bot);
                    bots.insert(bot_number, new_bot);
                }
            } else {
                // "bot N gives ..."
                let (bot_number, rule) = parse_rule(line);
                rules.insert(bot_number, rule);
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

fn parse_rule(line: &str) -> (u32, Rule) {
    // println!("rule: {}", line);
    let mut components = line.split_whitespace();
    let bot_number = components
        .nth(1)
        .expect("expecting bot giver at position 1")
        .parse()
        .expect("expecting nubmer");
    let (low_receiver_type, low_receiver_number) = (
        components.nth(3).unwrap(),
        components.nth(0).unwrap().parse().unwrap(),
    );
    let (high_receiver_type, high_receiver_number) = (
        components.nth(3).unwrap(),
        components.nth(0).unwrap().parse().unwrap(),
    );
    let low_receiver = match low_receiver_type {
        "output" => Receiver::Output(low_receiver_number),
        "bot" => Receiver::Bot(low_receiver_number),
        _ => unreachable!(),
    };
    let high_receiver = match high_receiver_type {
        "output" => Receiver::Output(high_receiver_number),
        "bot" => Receiver::Bot(high_receiver_number),
        _ => unreachable!(),
    };

    (bot_number, [low_receiver, high_receiver])
}
