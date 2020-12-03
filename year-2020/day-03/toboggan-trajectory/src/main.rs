static INPUT_DATA: &str = include_str!("input.txt");

fn check_slope(down: usize, right: usize) -> u64 {

    let mut trees_count = 0;
    let mut shift = right;

    for line in INPUT_DATA.split("\n").skip(down).step_by(down) {
        if line.chars().nth(shift).unwrap() == '#' {
            trees_count += 1;
        }
        shift = (shift + right) % line.len();
    }

    trees_count
}

fn main() {

    println!("{}", check_slope(1, 3));
    println!("{}", check_slope(1, 1) * check_slope(1, 3) * check_slope(1, 5) * check_slope(1, 7) * check_slope(2, 1));
}
