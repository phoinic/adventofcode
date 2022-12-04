use std::ops::Range;

static INPUT_DATA: &str = include_str!("input.txt");

fn read_input() -> Vec<(Range<u32>, Range<u32>)> {

    INPUT_DATA.split("\n").map(|str| {

        let pair_range = str.split(",").map(|str_pair| {
            let mut pair = str_pair.split("-");
            let start = pair.next().unwrap().parse::<u32>().unwrap();
            let end = pair.next().unwrap().parse::<u32>().unwrap();
            return Range { start, end};
        }).collect::<Vec<Range<u32>>>();

        return (pair_range[0].clone(), pair_range[1].clone())

    }).collect()
}

fn main() {

    let input = read_input();

    println!("{:?}", input);

    let round1 = input.iter().map(|pair| {
        if (pair.0.start >= pair.1.start && pair.0.end <= pair.1.end) || (pair.1.start >= pair.0.start && pair.1.end <= pair.0.end) { 1 } else { 0 }
    }).sum::<u32>();

    println!("Round 1: {}", round1);

    let round2 = input.iter().map(|pair| {
        if (pair.0.start <= pair.1.end && pair.0.end >= pair.1.start) || (pair.1.start <= pair.0.end && pair.1.end >= pair.0.start) { 1 } else { 0 }
    }).sum::<u32>();

    println!("Round 2: {}", round2);
}
