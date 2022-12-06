use array_tool::vec::Uniq;

static INPUT_DATA: &str = include_str!("input.txt");

fn read_input() -> Vec<char> {
    return INPUT_DATA.chars().collect::<Vec<_>>();
}

fn get_packet_start(input: &Vec<char>, amount: usize) -> Option<usize> {
    for i in amount..input.len() {
        let group = input[i - amount..i].to_vec();
        if group.is_unique() {
            return Some(i);
        }
    }
    None
}

fn main() {
    let input = read_input();

    println!("{:?}", input);

    let round1 = get_packet_start(&input, 4);
    let round2 = get_packet_start(&input, 14);

    println!("Round 1: {}", round1.unwrap());
    println!("Round 2: {}", round2.unwrap());
}
