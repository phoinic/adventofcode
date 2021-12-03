static INPUT_DATA: &str = include_str!("input.txt");

fn analyze(input: &Vec<&Vec<bool>>) -> Vec<(u64, u64)> {
    let mut analyzer = vec![];

    for seq in input.iter() {
        for (i, bit) in seq.iter().enumerate() {
            if analyzer.len() <= i {
                analyzer.push((0, 0));
            }
            match bit {
                false => analyzer[i].0 += 1,
                true => analyzer[i].1 += 1,
            }
        }
    }

    return analyzer;
}

fn compute_rate(bits: &Vec<bool>) -> u64 {
    bits.iter()
        .rev()
        .enumerate()
        .map(|(i, &val)| if val { 1 << i } else { 0 })
        .sum()
}

fn main() {
    let input = INPUT_DATA
        .split("\n")
        .map(|str| {
            let mut seq = vec![];
            for ch in str.chars() {
                match ch {
                    '0' => seq.push(false),
                    '1' => seq.push(true),
                    _ => panic!("Not a bit"),
                }
            }
            seq
        })
        .collect::<Vec<_>>();

    let analyzer = analyze(&input.iter().collect::<Vec<_>>());

    let mut gamma_bits = vec![];
    let mut epsilon_bits = vec![];

    for (zero, one) in analyzer.iter() {
        if zero > one {
            gamma_bits.push(false);
            epsilon_bits.push(true);
        } else {
            gamma_bits.push(true);
            epsilon_bits.push(false);
        }
    }

    println!("Round 1, gamma_bits: {:?}", gamma_bits);
    println!("Round 1, epsilon_bits: {:?}", epsilon_bits);

    let gamma_rate = compute_rate(&gamma_bits);
    let epsilon_rate = compute_rate(&epsilon_bits);

    println!("Round 1, gamma_rate = {}", gamma_rate);
    println!("Round 1, epsilon_rate = {}", epsilon_rate);

    let power_consumption = gamma_rate * epsilon_rate;

    println!("Round 1: {}", power_consumption);

    let mut oxygen_filtered_input = input.iter().collect::<Vec<_>>();
    let mut co2_filtered_input = input.iter().collect::<Vec<_>>();

    for i in 0..analyzer.len() {
        if oxygen_filtered_input.len() > 1 {
            let analyzer = analyze(&oxygen_filtered_input);
            let (zero, one) = analyzer[i];
            oxygen_filtered_input = oxygen_filtered_input
                .clone()
                .iter()
                .filter(|seq| (one >= zero && seq[i]) || (one < zero && !seq[i]))
                .map(|&seq| seq)
                .collect::<Vec<_>>();
        }
        if co2_filtered_input.len() > 1 {
            let analyzer = analyze(&co2_filtered_input);
            let (zero, one) = analyzer[i];
            co2_filtered_input = co2_filtered_input
                .clone()
                .iter()
                .filter(|seq| (one >= zero && !seq[i]) || (one < zero && seq[i]))
                .map(|&seq| seq)
                .collect::<Vec<_>>();
        }
    }

    let oxygen_bits = oxygen_filtered_input[0];
    let co2_bits = co2_filtered_input[0];

    println!("Round 2, oxygen_bits: {:?}", oxygen_bits);
    println!("Round 2, co2_bits: {:?}", co2_bits);

    let oxygen_rate = compute_rate(&oxygen_bits);
    let co2_rate = compute_rate(&co2_bits);

    println!("Round 2, oxygen_rate = {}", oxygen_rate);
    println!("Round 2, co2_rate = {}", co2_rate);

    let life_support_rating = oxygen_rate * co2_rate;

    println!("Round 2: {}", life_support_rating);
}
