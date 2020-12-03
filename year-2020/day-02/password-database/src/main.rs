use regex::Regex;

static INPUT_DATA: &str = include_str!("input.txt");

fn main() {
    let re = Regex::new(r"^([0-9]+)\-([0-9]+)\s([a-z]):\s([a-z]+)$").unwrap();

    let data = INPUT_DATA.split("\n");

    let mut valid1_count = 0;
    let mut valid2_count = 0;

    for str in data {

        let cap = re.captures_iter(str).nth(0).unwrap();

        let min_count = cap[1].parse::<u32>().unwrap();
        let max_count = cap[2].parse::<u32>().unwrap();
        let ch = cap[3].chars().nth(0).unwrap();
        let password = &cap[4];

        let mut ch_count: u32 = 0;

        for p_ch in password.chars() {
            if p_ch == ch {
                ch_count = ch_count + 1;
            }
        }

        if ch_count >= min_count && ch_count <= max_count {
            valid1_count = valid1_count + 1;
        }

        let pos1 = min_count as usize - 1;
        let pos2 = max_count as usize - 1;

        let ch1 = password.chars().nth(pos1).unwrap();
        let ch2 = password.chars().nth(pos2).unwrap();

        if (ch1 == ch && ch2 != ch) || (ch2 == ch && ch1 != ch) {
            valid2_count = valid2_count + 1;
        }
    }

    println!("{}", valid1_count);
    println!("{}", valid2_count);
}
