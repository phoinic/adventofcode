static INPUT_DATA: &str = include_str!("input.txt");

fn main() {
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];

    for str in INPUT_DATA.split("\n") {
        let values: Vec<&str> = str.split_whitespace().collect();

        left.push(str::parse::<u32>(values[0]).unwrap());
        right.push(str::parse::<u32>(values[1]).unwrap());
    }

    left.sort();
    right.sort();

    let round1 = left
        .iter()
        .zip(right.iter())
        .map(|(&a, &b)| ((a as i32) - (b as i32)).abs() as u32)
        .sum::<u32>();

    println!("Round1: {}", round1);

    let round2 = left
        .iter()
        .map(|&a| right.iter().filter(|&b| a == *b).count() as u32 * a)
        .sum::<u32>();

    println!("Round2: {}", round2);
}
