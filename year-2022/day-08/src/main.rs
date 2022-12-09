use crate::State::Visible;

static INPUT_DATA: &str = include_str!("input.txt");

#[derive(Debug, Clone, Copy)]
enum State {
    Invisible,
    Visible,
}

fn read_input() -> Vec<Vec<(i8, State)>> {
    return INPUT_DATA
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|char| (char.to_digit(10).unwrap() as i8, State::Invisible))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
}

fn main() {
    let mut input = read_input();

    println!("{:?}", input);

    let grid_size = input.len();

    for r in 0..grid_size {
        let mut max_h = -1;
        for c in 0..grid_size {
            if input[r][c].0 > max_h {
                max_h = input[r][c].0;
                input[r][c].1 = Visible;
            }
        }
    }

    for r in 0..grid_size {
        let mut max_h = -1;
        for c in (0..grid_size).rev() {
            if input[r][c].0 > max_h {
                max_h = input[r][c].0;
                input[r][c].1 = Visible;
            }
        }
    }

    for c in 0..grid_size {
        let mut max_h = -1;
        for r in 0..grid_size {
            if input[r][c].0 > max_h {
                max_h = input[r][c].0;
                input[r][c].1 = Visible;
            }
        }
    }

    for c in 0..grid_size {
        let mut max_h = -1;
        for r in (0..grid_size).rev() {
            if input[r][c].0 > max_h {
                max_h = input[r][c].0;
                input[r][c].1 = Visible;
            }
        }
    }

    println!("{:?}", input);

    let round1 = input
        .iter()
        .flatten()
        .map(|(_, state)| match state {
            State::Visible => 1,
            State::Invisible => 0,
        })
        .sum::<usize>();

    println!("Round 1: {}", round1);

    let mut round2 = 0;

    for r in 1..grid_size - 1 {
        for c in 1..grid_size - 1 {
            let mut score = 1;

            let mut count = 0;
            for cn in c + 1..grid_size {
                count += 1;
                if input[r][cn].0 >= input[r][c].0 {
                    break;
                }
            }
            score *= count;

            let mut count = 0;
            for cn in (0..c).rev() {
                count += 1;
                if input[r][cn].0 >= input[r][c].0 {
                    break;
                }
            }
            score *= count;

            let mut count = 0;
            for rn in r + 1..grid_size {
                count += 1;
                if input[rn][c].0 >= input[r][c].0 {
                    break;
                }
            }
            score *= count;

            let mut count = 0;
            for rn in (0..r).rev() {
                count += 1;
                if input[rn][c].0 >= input[r][c].0 {
                    break;
                }
            }
            score *= count;

            if score > round2 {
                round2 = score
            }
        }
    }

    println!("Round 2: {:?}", round2);
}
