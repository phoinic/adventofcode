static INPUT_DATA: &str = include_str!("input.txt");

fn main() {
    let input = INPUT_DATA.split("\n").collect::<Vec<&str>>();
    let positions = input[0]
        .split(",")
        .into_iter()
        .map(|i| i.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let min_pos: u64 = *positions.iter().min().unwrap();
    let max_pos: u64 = *positions.iter().max().unwrap();

    let round1 = (min_pos..max_pos)
        .into_iter()
        .map(|goal| {
            positions
                .iter()
                .map(|&pos| if pos > goal { pos - goal } else { goal - pos })
                .sum::<u64>()
        })
        .min()
        .unwrap();

    println!("Round 1: {}", round1);

    let round2 = (min_pos..max_pos)
        .into_iter()
        .map(|goal| {
            positions
                .iter()
                .map(|&pos| {
                    let len = if pos > goal { pos - goal } else { goal - pos };
                    let fuel = (1 + len) * len / 2;
                    fuel
                })
                .sum::<u64>()
        })
        .min()
        .unwrap();

    println!("Round 2: {}", round2);
}
