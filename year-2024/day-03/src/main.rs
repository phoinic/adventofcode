use regex::Regex;

static INPUT_DATA: &str = include_str!("input.txt");

fn main() {
    let mut round1 = 0;
    let mut round2 = 0;

    let re1 = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    for (_, [a, b]) in re1.captures_iter(INPUT_DATA).map(|c| c.extract()) {
        round1 += str::parse::<i64>(a).unwrap() * str::parse::<i64>(b).unwrap();
    }

    let re2 = Regex::new(r"(mul|don't|do)(:?\((\d{1,3}),(\d{1,3})\))?").unwrap();

    let mut is_do = true;

    for cap in re2.captures_iter(INPUT_DATA) {
        let parts = cap.iter().collect::<Vec<_>>();
        match parts[1].unwrap().as_str() {
            "mul" => {
                round2 += if parts[3].is_some() && parts[4].is_some() && is_do {
                    str::parse::<i64>(parts[3].unwrap().as_str()).unwrap()
                        * str::parse::<i64>(parts[4].unwrap().as_str()).unwrap()
                } else {
                    0
                }
            }
            "don't" => is_do = false,
            "do" => is_do = true,
            _ => {}
        };
    }

    println!("Round1: {}", round1);
    println!("Round2: {}", round2);
}
