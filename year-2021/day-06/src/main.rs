static INPUT_DATA: &str = include_str!("input.txt");

fn main() {
    let input = INPUT_DATA.split("\n").collect::<Vec<&str>>();
    let mut fishes = input[0]
        .split(",")
        .into_iter()
        .map(|i| i.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let days = 256;
    let mut population = vec![0u64; days];

    for fish in fishes.iter_mut() {
        for day in (*fish as usize..population.len()).step_by(7) {
            population[day] += 1;
        }
    }

    for cday in 0..population.len() {
        let count = population[cday];
        for day in (cday + 9..population.len()).step_by(7) {
            population[day] += count;
        }
    }

    println!(
        "Round 1: {:?}",
        population[0..80].iter().sum::<u64>() + fishes.len() as u64
    );

    println!(
        "Round 2: {:?}",
        population[0..256].iter().sum::<u64>() + fishes.len() as u64
    );
}
