#![feature(let_chains)]

use std::cmp::Ordering;
use std::collections::HashMap;

mod types;
use types::*;

// the task is to find bot which holds this two microchips
const TASK: (Microchip, Microchip) = (61, 17);

fn main() {
    println!("--- Day 10: Balance Bots ---");

    let input_file = std::env::args()
        .nth(1)
        .expect("expecting input filename as an argument");

    let (mut bots, rules) = read_input(&input_file);

    let mut outputs = HashMap::new();
    let mut answer: u32 = 69420; // init with garbage

    start_simulation(&mut bots, &rules, &mut outputs, &mut answer);

    println!("{}", answer);
    println!(
        "{}",
        [0, 1, 2]
            .iter()
            .map(|idx| outputs.get(&idx).unwrap())
            .product::<u32>()
    );
}

fn start_simulation(bots: &mut Bots, rules: &Rules, outputs: &mut Outputs, answer: &mut u32) {
    let mut do_next: bool;

    'simulation: loop {
        do_next = false;

        let bot_numbers: Vec<u32> = bots.keys().map(|number| *number).collect();

        for bot_number in &bot_numbers {
            let bot = &bots
                .get(bot_number)
                .expect("expecting valid iteration")
                .clone();

            match bot {
                [Some(_), Some(_)] => {
                    let rule = rules.get(bot_number).expect("expecting rule for the bot");
                    apply_rule(rule, bot, bot_number, bots, answer, outputs);
                    bots.entry(*bot_number).and_modify(|item| *item = [None; 2]);

                    // simulation continues until we can't find bot with two microchips.
                    do_next = true;
                }
                [_, _] => {
                    // bots with less than two microchips are skipped.
                    continue;
                }
            }
        }
        if !do_next {
            break 'simulation;
        }
    }
}

fn process_receiver(
    receiver: &Receiver,
    bot: &Bot,
    bot_number: &u32,
    bots: &mut Bots,
    ordering: Ordering,
    answer: &mut u32,
    outputs: &mut Outputs,
) {
    match *receiver {
        Receiver::Output(number) => {
            outputs.insert(number, take(bot, *bot_number, ordering, answer));
        }
        Receiver::Bot(number) => {
            if let Some(receiver) = bots.get_mut(&number) {
                give(take(bot, *bot_number, ordering, answer), receiver);
            } else {
                let mut new_bot = [None; 2];
                give(take(bot, *bot_number, ordering, answer), &mut new_bot);
                bots.insert(number, new_bot);
            }
        }
    }
}

fn apply_rule(
    rule: &Rule,
    bot: &Bot,
    bot_number: &u32,
    bots: &mut Bots,
    answer: &mut u32,
    outputs: &mut Outputs,
) {
    [Ordering::Less, Ordering::Greater]
        .iter()
        .enumerate()
        .for_each(|(idx, ordering)| {
            process_receiver(
                &rule[idx], bot, bot_number, bots, *ordering, answer, outputs,
            );
        });
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

fn take(bot: &Bot, bot_number: u32, ordering: Ordering, answer: &mut u32) -> Microchip {
    if let Some(hand1) = bot[0]
        && let Some(hand2) = bot[1]
    {
        if (hand1, hand2) == TASK || (hand2, hand1) == TASK {
            *answer = bot_number;
        }
        return match hand1.cmp(&hand2) == ordering {
            true => hand1,
            false => hand2,
        };
    }
    unreachable!("cant take from bot with less than two  microchips. wrong input");
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
                let (bot_number, microchip) = parse_bot(line);
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
    let mut components = line.split_whitespace();

    let microchip = components
        .nth(1)
        .expect("expecting microchip at position 1")
        .parse()
        .unwrap();

    let bot_number = components
        .nth(3)
        .expect("expecting bot number at position 5")
        .parse()
        .unwrap();

    (bot_number, microchip)
}

fn parse_rule(line: &str) -> (u32, Rule) {
    let mut components = line.split_whitespace();

    let bot_number = components
        .nth(1)
        .expect("expecting bot number at position 1")
        .parse()
        .unwrap();

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
