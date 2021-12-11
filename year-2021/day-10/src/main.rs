static INPUT_DATA: &str = include_str!("input.txt");

fn main() {
    let input = INPUT_DATA.split("\n").into_iter().collect::<Vec<_>>();

    let mut round1 = 0;
    let mut round2 = vec![];

    for line in input.iter() {
        let mut analyzer = vec![];
        let mut is_valid = true;
        for ch in line.chars() {
            if ['(', '[', '<', '{'].contains(&ch) {
                analyzer.push(ch);
            }
            if [')', ']', '>', '}'].contains(&ch) {
                let cch = analyzer.pop().unwrap();
                if (ch == ')' && cch != '(')
                    || (ch == ']' && cch != '[')
                    || (ch == '>' && cch != '<')
                    || (ch == '}' && cch != '{')
                {
                    round1 += match ch {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        _ => 0,
                    };
                    is_valid = false;
                    break;
                }
            }
        }
        if is_valid && analyzer.len() > 0 {
            let mut score: u64 = 0;
            while analyzer.len() > 0 {
                let ch = analyzer.pop().unwrap();
                score *= 5;
                score += match ch {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => 0,
                };
            }
            round2.push(score);
        }
    }

    round2.sort();

    println!("Round 1: {}", round1);
    println!("Round 2: {}", round2[round2.len() / 2]);
}
