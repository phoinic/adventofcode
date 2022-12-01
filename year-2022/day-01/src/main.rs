static INPUT_DATA: &str = include_str!("input.txt");

fn main() {
    let mut elves: Vec<Vec<i64>> = vec![];
    let mut elf: Vec<i64> = vec![];

    for str in INPUT_DATA.split("\n") {
        if str == "" {
            elves.push(elf);
            elf = vec![];
        } else {
            elf.push(str.parse::<i64>().unwrap());
        }
    }

    if elf.len() > 0 {
        elves.push(elf);
    }

    println!("{:?}", elves);

    let mut sums: Vec<i64> = elves.iter().map(|elf| elf.iter().sum()).collect::<Vec<_>>();
    sums.sort_by(|a, b| b.cmp(a));

    println!("Round 1: {}", sums[0]);
    println!("Round 2: {}", sums[0] + sums[1] + sums[2]);
}
