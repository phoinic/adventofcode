static INPUT_DATA: &str = include_str!("input.txt");

use std::collections::HashMap;

#[derive(Debug)]
struct Packet {
    pub patterns: Vec<u8>,
    pub digits: Vec<u8>,
}

fn count_bits(bits: u8) -> u8 {
    let mut count = 0;
    let mut bits = bits;
    while bits > 0 {
        count += bits % 2;
        bits >>= 1;
    }
    count
}

fn encode_to_bit(signal: &str) -> u8 {
    signal
        .chars()
        .into_iter()
        .map(|c| match c {
            'a' => 1,
            'b' => 1 << 1,
            'c' => 1 << 2,
            'd' => 1 << 3,
            'e' => 1 << 4,
            'f' => 1 << 5,
            'g' => 1 << 6,
            _ => 0,
        })
        .reduce(|a, b| a | b)
        .unwrap()
}

fn build_map<'a>(packet: &'a Packet) -> HashMap<u8, u8> {
    let mut map = HashMap::<u8, u8>::new();

    let signals = packet
        .patterns
        .iter()
        .chain(packet.digits.iter())
        .map(|&signal| signal)
        .collect::<Vec<_>>();

    while map.len() < 10 {
        for signal in signals.iter() {
            match count_bits(*signal) {
                2 => {
                    map.insert(1, *signal);
                }
                3 => {
                    map.insert(7, *signal);
                }
                4 => {
                    map.insert(4, *signal);
                }
                7 => {
                    map.insert(8, *signal);
                }
                6 => {
                    if map.contains_key(&4) && !map.contains_key(&9) {
                        if *map.get(&4).unwrap() | *signal == *signal {
                            map.insert(9, *signal);
                        }
                    }
                    if map.contains_key(&8) {
                        if map.contains_key(&5) && !map.contains_key(&0) {
                            if *map.get(&5).unwrap() | *signal == *map.get(&8).unwrap() {
                                map.insert(0, *signal);
                            }
                        }
                        if map.contains_key(&7) && !map.contains_key(&6) {
                            if *map.get(&7).unwrap() | *signal == *map.get(&8).unwrap() {
                                map.insert(6, *signal);
                            }
                        }
                    }
                }
                5 => {
                    if map.contains_key(&7) && !map.contains_key(&3) {
                        if *map.get(&7).unwrap() | *signal == *signal {
                            map.insert(3, *signal);
                        }
                    }
                    if map.contains_key(&8) {
                        if map.contains_key(&4) && !map.contains_key(&2) {
                            if *map.get(&4).unwrap() | *signal == *map.get(&8).unwrap() {
                                map.insert(2, *signal);
                            }
                        }
                    }
                    if map.contains_key(&9) {
                        if map.contains_key(&1) && !map.contains_key(&5) {
                            if *map.get(&1).unwrap() | *signal == *map.get(&9).unwrap() {
                                map.insert(5, *signal);
                            }
                        }
                    }
                }
                _ => {}
            };
        }
    }

    let mut map_decode = HashMap::<u8, u8>::new();

    for (key, value) in map.iter() {
        map_decode.insert(*value, *key);
    }

    map_decode
}

fn main() {
    let input = INPUT_DATA
        .split("\n")
        .into_iter()
        .map(|line| {
            let mut tmp = line.split(" | ");
            Packet {
                patterns: tmp
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(encode_to_bit)
                    .collect::<Vec<_>>(),
                digits: tmp
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(encode_to_bit)
                    .collect::<Vec<_>>(),
            }
        })
        .collect::<Vec<_>>();

    let round1 = input
        .iter()
        .map(|packet| {
            packet
                .digits
                .iter()
                .map(|&signal| {
                    if [2u8, 3, 4, 7].contains(&count_bits(signal)) {
                        1
                    } else {
                        0
                    }
                })
                .sum::<u64>()
        })
        .sum::<u64>();

    println!("Round 1: {}", round1);

    let round2 = input
        .iter()
        .map(|packet| {
            let map = build_map(packet);
            let value = packet
                .digits
                .iter()
                .rev()
                .enumerate()
                .map(|(i, digit)| map[digit] as u64 * 10_u64.pow(i as u32) as u64)
                .sum::<u64>();
            value
        })
        .sum::<u64>();

    println!("Round 2: {}", round2);
}
