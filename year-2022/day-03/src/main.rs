use array_tool::vec::Intersect;

static INPUT_DATA: &str = include_str!("input.txt");

static ITEMS_PRIORITY: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn read_input() -> Vec<Vec<char>> {

    INPUT_DATA.split("\n").map(|str| {
        str.chars().collect()
    }).collect()
}

fn main() {

    let input = read_input();

    println!("{:?}", input);

    let round1: usize = input.iter().map(|items| {
        let p1 = items[0..items.len()/2].to_vec();
        let p2 = items[items.len()/2..].to_vec();
        let common_items = p1.intersect(p2);
        common_items.iter().map(|&item| {
            ITEMS_PRIORITY.find(item).unwrap() + 1
        }).sum::<usize>()
    }).sum::<usize>();

    println!("Round 1: {}", round1);

    let round2: usize = input.chunks(3).map(|chunk| {
        let elves = chunk.to_vec();
        let common_items = elves[0].intersect(elves[1].clone()).intersect(elves[2].clone());
        common_items.iter().map(|&item| {
            ITEMS_PRIORITY.find(item).unwrap() + 1
        }).sum::<usize>()
    }).sum::<usize>();

    println!("Round 2: {}", round2);
}
