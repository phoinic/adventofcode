static INPUT_DATA: &str = include_str!("input.txt");

fn get_digit(str: &str, i: usize) -> Option<u32> {
    let c = str.chars().nth(i).unwrap();
    if c.is_digit(10) {
        return Some(c.to_digit(10).unwrap());
    }
    if i as i32 <= str.len() as i32 - 3 {
        if str[i..i + 3] == "one".to_owned() {
            return Some(1);
        }
        if str[i..i + 3] == "two".to_owned() {
            return Some(2);
        }
        if str[i..i + 3] == "six".to_owned() {
            return Some(6);
        }
    }
    if i as i32 <= str.len() as i32 - 4 {
        if str[i..i + 4] == "four".to_owned() {
            return Some(4);
        }
        if str[i..i + 4] == "five".to_owned() {
            return Some(5);
        }
        if str[i..i + 4] == "nine".to_owned() {
            return Some(9);
        }
    }
    if i as i32 <= str.len() as i32 - 5 {
        if str[i..i + 5] == "three".to_owned() {
            return Some(3);
        }
        if str[i..i + 5] == "seven".to_owned() {
            return Some(7);
        }
        if str[i..i + 5] == "eight".to_owned() {
            return Some(8);
        }
    }
    None
}

fn main() {
    let mut round1 = 0u32;
    let mut round2 = 0u32;

    for str in INPUT_DATA.split("\n") {
        let mut first: Option<u32> = None;
        let mut last: Option<u32> = None;
        let chars = str.chars().into_iter().collect::<Vec<_>>();
        for c in chars {
            if c.is_digit(10) {
                last = Some(c.to_digit(10).unwrap());
                if first.is_none() {
                    first = last;
                }
            }
        }
        let calibration = first.unwrap_or(0) * 10 + last.unwrap_or(0);
        round1 += calibration;
    }

    println!("Round 1: {}", round1);

    for str in INPUT_DATA.split("\n") {
        let mut first: Option<u32> = None;
        let mut last: Option<u32> = None;
        for i in 0..str.len() {
            let d = get_digit(str, i);
            if d.is_some() {
                last = d;
                if first.is_none() {
                    first = last;
                }
            }
        }
        let calibration = first.unwrap_or(0) * 10 + last.unwrap_or(0);
        round2 += calibration;
    }

    println!("Round 2: {}", round2);
}
