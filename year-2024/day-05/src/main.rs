use std::{cmp::Ordering, collections::BTreeMap};

static INPUT_DATA: &str = include_str!("input.txt");

fn read_input() -> (BTreeMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let mut is_produce = false;

    let mut rules = BTreeMap::<u32, Vec<u32>>::new();
    let mut pages_sequences = Vec::<Vec<u32>>::new();

    for str in INPUT_DATA.split("\n") {
        if str == "" {
            is_produce = true;
            continue;
        }

        if is_produce {
            pages_sequences.push(
                str.split(",")
                    .map(|v| str::parse::<u32>(v).unwrap())
                    .collect(),
            );
        } else {
            let pair = str
                .split("|")
                .map(|v| str::parse::<u32>(v).unwrap())
                .collect::<Vec<u32>>();
            if rules.contains_key(&pair[0]) {
                rules.get_mut(&pair[0]).unwrap().push(pair[1]);
            } else {
                rules.insert(pair[0], vec![pair[1]]);
            }
        }
    }

    (rules, pages_sequences)
}

fn is_correct(rules: &BTreeMap<u32, Vec<u32>>, pages: &Vec<u32>) -> bool {
    for i in 1..pages.len() {
        for j in 0..i {
            if rules.contains_key(&pages[i]) && rules.get(&pages[i]).unwrap().contains(&pages[j]) {
                return false;
            }
        }
    }

    true
}

fn resort(rules: &BTreeMap<u32, Vec<u32>>, pages: &Vec<u32>) -> Vec<u32> {
    let mut resorted = pages.clone();

    resorted.sort_by(|a, b| {
        if rules.contains_key(a) && rules.get(a).unwrap().contains(b) {
            return Ordering::Less;
        }
        if rules.contains_key(b) && rules.get(b).unwrap().contains(a) {
            return Ordering::Greater;
        }
        return Ordering::Equal;
    });

    resorted
}

fn main() {
    let mut round1 = 0;
    let mut round2 = 0;

    let (rules, pages_sequences) = read_input();

    for pages in pages_sequences.iter() {
        if is_correct(&rules, pages) {
            round1 += pages[pages.len() / 2];
        } else {
            let resorted = resort(&rules, pages);
            round2 += resorted[resorted.len() / 2];
        }
    }

    println!("Round1: {}", round1);
    println!("Round2: {}", round2);
}
