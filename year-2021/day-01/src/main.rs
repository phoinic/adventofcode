static INPUT_DATA: &str = include_str!("input.txt");

fn main() {
    let input = INPUT_DATA.split("\n")
        .map(|str| {
            str.to_string().parse::<i64>().unwrap()
        }).collect::<Vec<_>>();

    let mut count = 0;
    let mut prev: Option<i64> = None;
    for &val in input.iter() {
        if prev.is_some() && prev.unwrap() < val {
            count += 1;
        }
        prev = Some(val);
    }

    println!("Round 1: {}", count);

    let mut count = 0;
    let mut prev: Option<i64> = None;
    for i in 0..(input.len() - 2) {
        let val = input[i] + input[i + 1] + input[i + 2];
        if prev.is_some() && prev.unwrap() < val {
            count += 1;
        }
        prev = Some(val);
    }

    println!("Round 2: {}", count);
}
