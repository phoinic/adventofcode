use regex::Regex;
use std::collections::VecDeque;

static INPUT_DATA: &str = include_str!("input.txt");

fn read_input() -> (Vec<VecDeque<char>>, Vec<VecDeque<char>>) {
    enum InputPart {
        Stacks,
        Moves,
    }

    let mut stacks1 = vec![];
    let mut stacks2 = vec![];
    let mut input_part: InputPart = InputPart::Stacks;

    let numbers_re = Regex::new(r"^[\s0-9]+$").unwrap();
    let move_re = Regex::new(r"(move\s|from\s|to\s)").unwrap();

    for str in INPUT_DATA.split("\n") {
        if str == "" {
            input_part = InputPart::Moves;
            stacks2 = stacks1.clone();
            continue;
        }

        if numbers_re.is_match(str) {
            continue;
        }

        match input_part {
            InputPart::Stacks => {
                for (index, item) in str.chars().collect::<Vec<_>>().chunks(4).enumerate() {
                    if stacks1.len() <= index {
                        stacks1.push(VecDeque::new());
                    }
                    if item[1] != ' ' {
                        stacks1[index].push_front(item[1]);
                    }
                }
            }
            InputPart::Moves => {
                let stripped_str = move_re.replace_all(str, "");
                let tokens = stripped_str.split(" ").collect::<Vec<_>>();
                let count = tokens[0].parse::<usize>().unwrap();
                let from = tokens[1].parse::<usize>().unwrap() - 1;
                let to = tokens[2].parse::<usize>().unwrap() - 1;

                let mut pack = vec![];

                for _ in 0..count {
                    let m1 = stacks1[from].pop_back().unwrap();
                    stacks1[to].push_back(m1);

                    let m2 = stacks2[from].pop_back().unwrap();
                    pack.push(m2)
                }

                for &item in pack.iter().rev() {
                    stacks2[to].push_back(item);
                }
            }
        };
    }

    (stacks1, stacks2)
}

fn main() {
    let input = read_input();

    println!("{:?}", input);

    let round1: String = input
        .0
        .iter()
        .map(|stack| {
            if stack.len() > 0 {
                *stack.back().unwrap()
            } else {
                '\0'
            }
        })
        .collect();

    let round2: String = input
        .1
        .iter()
        .map(|stack| {
            if stack.len() > 0 {
                *stack.back().unwrap()
            } else {
                '\0'
            }
        })
        .collect();

    println!("Round 1: {}", round1);
    println!("Round 2: {}", round2);
}
