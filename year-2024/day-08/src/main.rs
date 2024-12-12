use std::collections::BTreeMap;

static INPUT_DATA: &str = include_str!("input.txt");

type MapType = Vec<Vec<char>>;

fn print_map(map: &MapType) {
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            print!("{}", map[row][col]);
        }
        println!();
    }
    println!();
}

fn main() {
    let mut round1 = 0;
    let mut round2 = 0;

    let mut map: MapType = vec![];

    for str in INPUT_DATA.split("\n") {
        map.push(str.chars().collect());
    }

    let mut round1_map = map.clone();
    let mut round2_map = map.clone();

    let mut nodes = BTreeMap::<char, Vec<(usize, usize)>>::new();

    for row in 0..map.len() {
        for col in 0..map[row].len() {
            let c = map[col][row];

            if c == '.' {
                continue;
            }

            if !nodes.contains_key(&c) {
                nodes.insert(c, vec![]);
            }

            nodes.get_mut(&c).unwrap().push((row, col));
        }
    }

    for (_, antenas) in nodes {
        for i in 0..antenas.len() {
            let (row1, col1) = antenas[i];
            for k in 0..antenas.len() {
                if k == i {
                    continue;
                }

                let (row2, col2) = antenas[k];

                let dr = row1 as i64 - row2 as i64;
                let dc = col1 as i64 - col2 as i64;

                let round1_row = row1 as i64 + dr;
                let round1_col = col1 as i64 + dc;

                if round1_row >= 0
                    && round1_row < map.len() as i64
                    && round1_col >= 0
                    && round1_col < map[0].len() as i64
                {
                    if round1_map[round1_row as usize][round1_col as usize] != '#' {
                        round1_map[round1_row as usize][round1_col as usize] = '#';
                        round1 += 1;
                    }
                }

                let round1_row = row2 as i64 - dr;
                let round1_col = col2 as i64 - dc;

                if round1_row >= 0
                    && round1_row < map.len() as i64
                    && round1_col >= 0
                    && round1_col < map[0].len() as i64
                {
                    if round1_map[round1_row as usize][round1_col as usize] != '#' {
                        round1_map[round1_row as usize][round1_col as usize] = '#';
                        round1 += 1;
                    }
                }

                let mut round2_row = row1 as i64;
                let mut round2_col = col1 as i64;

                while round2_row >= 0
                    && round2_row < map.len() as i64
                    && round2_col >= 0
                    && round2_col < map[0].len() as i64
                {
                    if round2_map[round2_row as usize][round2_col as usize] != '#' {
                        round2_map[round2_row as usize][round2_col as usize] = '#';
                        round2 += 1;
                    }

                    round2_row += dr;
                    round2_col += dc;
                }

                let mut round2_row = row2 as i64;
                let mut round2_col = col2 as i64;

                while round2_row >= 0
                    && round2_row < map.len() as i64
                    && round2_col >= 0
                    && round2_col < map[0].len() as i64
                {
                    if round2_map[round2_row as usize][round2_col as usize] != '#' {
                        round2_map[round2_row as usize][round2_col as usize] = '#';
                        round2 += 1;
                    }

                    round2_row -= dr;
                    round2_col -= dc;
                }
            }
        }
    }

    print_map(&round1_map);
    println!("Round1: {}", round1);

    print_map(&round2_map);
    println!("Round2: {}", round2);
}
