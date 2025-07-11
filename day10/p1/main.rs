#![feature(let_chains)]

use std::cmp::Ordering;
use std::collections::HashMap;

mod types;
use types::*;

// the task is to find bot which holds this two microchips
const TASK: (Microchip, Microchip) = (61, 17);

struct SimState<'a> {
    bots: &'a mut Bots,
    outputs: &'a mut Outputs,
    answer: &'a mut u32,
}

fn main() {
    println!("--- Day 10: Balance Bots ---");

    let input_file = std::env::args()
        .nth(1)
        .expect("expecting input filename as an argument");

    let (mut bots, rules) = read_input(&input_file);

    let mut outputs = HashMap::new();
    let mut answer: u32 = 69420; // init with garbage

    let mut sim_state: SimState = SimState {
        bots: &mut bots,
        outputs: &mut outputs,
        answer: &mut answer,
    };

    start_simulation(&rules, &mut sim_state);

    println!("{}", answer);
    println!(
        "{}",
        [0, 1, 2]
            .iter()
            .map(|idx| outputs.get(&idx).unwrap())
            .product::<u32>()
    );
}

fn start_simulation(rules: &Rules, sim_state: &mut SimState) {
    let mut do_next: bool;

    'simulation: loop {
        do_next = false;

        let bot_ids: Vec<u32> = sim_state.bots.keys().map(|id| *id).collect();

        for bot_id in bot_ids {
            let bot = &sim_state
                .bots
                .get(&bot_id)
                .expect("expecting valid iteration")
                .clone();

            match bot {
                [Some(_), Some(_)] => {
                    let rule = rules.get(&bot_id).expect("expecting rule for the bot");
                    apply_rule(rule, bot, bot_id, sim_state);

                    // after we apply the rule, bot's hands must be free.
                    sim_state
                        .bots
                        .entry(bot_id)
                        .and_modify(|item| *item = [None; 2]);

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
    bot_id: u32,
    ordering: Ordering,
    sim_state: &mut SimState,
) {
    match *receiver {
        Receiver::Output(id) => {
            sim_state
                .outputs
                .insert(id, take(bot, bot_id, ordering, sim_state.answer));
        }
        Receiver::Bot(id) => {
            if let Some(receiver) = sim_state.bots.get_mut(&id) {
                give(take(bot, bot_id, ordering, sim_state.answer), receiver);
            } else {
                let mut new_bot = [None; 2];
                give(
                    take(bot, bot_id, ordering, sim_state.answer),
                    &mut new_bot,
                );
                sim_state.bots.insert(id, new_bot);
            }
        }
    }
}

fn apply_rule(rule: &Rule, bot: &Bot, bot_id: u32, sim_state: &mut SimState) {
    [Ordering::Less, Ordering::Greater]
        .iter()
        .enumerate()
        .for_each(|(idx, ordering)| {
            process_receiver(&rule[idx], bot, bot_id, *ordering, sim_state);
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

fn take(bot: &Bot, bot_id: u32, ordering: Ordering, answer: &mut u32) -> Microchip {
    if let Some(hand1) = bot[0]
        && let Some(hand2) = bot[1]
    {
        if (hand1, hand2) == TASK || (hand2, hand1) == TASK {
            *answer = bot_id;
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
                let (bot_id, microchip) = parse_bot(line);
                if let Some(bot) = bots.get_mut(&bot_id) {
                    give(microchip, bot);
                } else {
                    let mut new_bot = [None; 2];
                    give(microchip, &mut new_bot);
                    bots.insert(bot_id, new_bot);
                }
            } else {
                // "bot N gives ..."
                let (bot_id, rule) = parse_rule(line);
                rules.insert(bot_id, rule);
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

    let bot_id = components
        .nth(3)
        .expect("expecting bot id at position 5")
        .parse()
        .unwrap();

    (bot_id, microchip)
}

fn parse_rule(line: &str) -> (u32, Rule) {
    let mut components = line.split_whitespace();

    let bot_id = components
        .nth(1)
        .expect("expecting bot id at position 1")
        .parse()
        .unwrap();

    let (low_receiver_type, low_receiver_id) = (
        components.nth(3).unwrap(),
        components.nth(0).unwrap().parse().unwrap(),
    );

    let (high_receiver_type, high_receiver_id) = (
        components.nth(3).unwrap(),
        components.nth(0).unwrap().parse().unwrap(),
    );

    let low_receiver = match low_receiver_type {
        "output" => Receiver::Output(low_receiver_id),
        "bot" => Receiver::Bot(low_receiver_id),
        _ => unreachable!(),
    };

    let high_receiver = match high_receiver_type {
        "output" => Receiver::Output(high_receiver_id),
        "bot" => Receiver::Bot(high_receiver_id),
        _ => unreachable!(),
    };

    (bot_id, [low_receiver, high_receiver])
}
