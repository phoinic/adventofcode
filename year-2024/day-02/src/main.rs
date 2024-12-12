static INPUT_DATA: &str = include_str!("input.txt");

fn is_safe(level: &Vec<i64>) -> bool {
    level.windows(2).all(|w| (w[0] < w[1]) && (w[1] - w[0] < 4))
        || level.windows(2).all(|w| (w[0] > w[1]) && (w[0] - w[1] < 4))
}

fn try_safe(level: &Vec<i64>) -> bool {
    for i in 0..level.len() {
        if is_safe(
            &level
                .iter()
                .enumerate()
                .filter(|&(j, _)| j != i)
                .map(|(_, &a)| a)
                .collect::<Vec<i64>>(),
        ) {
            return true;
        }
    }

    false
}

fn main() {
    let mut round1 = 0;
    let mut round2 = 0;

    for str in INPUT_DATA.split("\n") {
        let level: Vec<i64> = str
            .split_whitespace()
            .map(|v| str::parse::<i64>(v).unwrap())
            .collect();

        let level_safe = is_safe(&level);

        round1 += if level_safe { 1 } else { 0 };
        round2 += if level_safe || try_safe(&level) { 1 } else { 0 };
    }

    println!("Round1: {}", round1);
    println!("Round2: {}", round2);
}
