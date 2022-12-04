static INPUT_DATA: &str = include_str!("input.txt");

enum Strategy {
    Round1,
    Round2
}

fn read_input() -> Vec<(char, char)> {

    let mut res = vec![];

    for str in INPUT_DATA.split("\n") {
        let parts = str.split(" ").map(|part| part.chars().next().unwrap()).collect::<Vec<_>>();
        let p1 = parts.get(0).unwrap().clone();
        let p2 = parts.get(1).unwrap().clone();

        res.push((p1, p2))
    }

    res
}

fn get_score(round: &(char, char), strategy: Strategy) -> u32 {
    use Strategy::*;

    match round.1 {
        'X' => {
            (match strategy { Round1 => 1, Round2 => 0 }) +
            (match round.0 {
                'A' => match strategy { Round1 => 3, Round2 => 3 },
                'B' => match strategy { Round1 => 0, Round2 => 1 },
                'C' => match strategy { Round1 => 6, Round2 => 2 },
                _ => panic!("Unexpected turn {:?}", round)
            })
        },
        'Y' => {
            (match strategy { Round1 => 2, Round2 => 3 }) +
            (match round.0 {
                'A' => match strategy { Round1 => 6, Round2 => 1 },
                'B' => match strategy { Round1 => 3, Round2 => 2 },
                'C' => match strategy { Round1 => 0, Round2 => 3 },
                _ => panic!("Unexpected turn {:?}", round)
            })
        },
        'Z' => {
            (match strategy { Round1 => 3, Round2 => 6 }) +
            (match round.0 {
                'A' => match strategy { Round1 => 0, Round2 => 2 },
                'B' => match strategy { Round1 => 6, Round2 => 3 },
                'C' => match strategy { Round1 => 3, Round2 => 1 },
                _ => panic!("Unexpected turn {:?}", round)
            })
        }
        _ => panic!("Unexpected turn {:?}", round)
    }
}

fn main() {

    let input = read_input();

    println!("{:?}", input);

    let round1 = input.iter().map(|round| get_score(round, Strategy::Round1)).sum::<u32>();
    let round2 = input.iter().map(|round| get_score(round, Strategy::Round2)).sum::<u32>();

    println!("Round 1: {}", round1);
    println!("Round 2: {}", round2);
}
