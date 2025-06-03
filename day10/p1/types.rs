use std::collections::HashMap;

pub type Microchip = u32;
// actually we don't need outputs for now, but let's keep them in case.
// ahaha :-) got it in part2
pub type Outputs = HashMap<u32, Microchip>; // each "output" bin holds one microchip

// Bot is basically described as a two microchips it holds.
pub type Bot = [Option<Microchip>; 2];
pub type Bots = HashMap<u32, Bot>;

#[derive(Debug)]
pub enum Receiver {
    Bot(u32),
    Output(u32),
}
// Rule is the way to describe who receives lower and higher values.
pub type Rule = [Receiver; 2];
pub type Rules = HashMap<u32, Rule>;
