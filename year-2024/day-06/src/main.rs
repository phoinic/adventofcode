use std::collections::{BTreeMap, HashSet};

static INPUT_DATA: &str = include_str!("input.txt");

static GUARD_SYMBOLS: &[char] = &['^', '>', 'v', '<'];

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

fn get_guard_pos(map: &MapType) -> (usize, usize) {
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if GUARD_SYMBOLS.contains(&map[row][col]) {
                return (row, col);
            }
        }
    }

    panic!("No guard on Map!");
}

fn move_guard(map: &mut MapType, row: usize, col: usize) -> u32 {
    let mut visited = BTreeMap::<(usize, usize), HashSet<char>>::new();

    let mut row = row;
    let mut col = col;
    let mut path = 0u32;

    loop {
        let guard = map[row][col];

        if visited.contains_key(&(row, col)) {
            if visited.get(&(row, col)).unwrap().contains(&guard) {
                return 0;
            }
        } else {
            visited.insert((row, col), HashSet::new());
        }

        visited.get_mut(&(row, col)).unwrap().insert(guard);

        let (dr, dc): (i32, i32) = match guard {
            '^' => (-1, 0),
            '>' => (0, 1),
            'v' => (1, 0),
            '<' => (0, -1),
            _ => panic!("Guard lost!"),
        };

        if (row as i32 + dr) < 0
            || (row as i32 + dr) == map.len() as i32
            || (col as i32 + dc) < 0
            || (col as i32 + dc) == map[row].len() as i32
        {
            map[row][col] = 'X';
            path += 1;
            break;
        }

        let next_row = (row as i32 + dr) as usize;
        let next_col = (col as i32 + dc) as usize;
        let next_cell = map[next_row][next_col];

        match next_cell {
            '#' => {
                let guard_index = GUARD_SYMBOLS.iter().position(|&c| c == guard).unwrap();
                map[row][col] = GUARD_SYMBOLS[(guard_index + 1) % 4];
                continue;
            }
            '.' => {
                path += 1;
            }
            _ => {}
        };

        map[row][col] = 'X';
        map[next_row][next_col] = guard;

        row = next_row;
        col = next_col;
    }

    path
}

fn main() {
    let mut map: MapType = vec![];

    for str in INPUT_DATA.split("\n") {
        map.push(str.chars().collect());
    }

    let (row, col) = get_guard_pos(&map);
    let guard = map[row][col];

    let round1 = move_guard(&mut map, row, col);

    let mut round2 = 0;

    map[row][col] = guard;

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if (i == col && j == col) || map[i][j] != 'X' {
                continue;
            }

            let mut map_looped = map.clone();
            map_looped[i][j] = '#';

            if move_guard(&mut map_looped, row, col) == 0 {
                round2 += 1;
            }
        }
    }

    println!("Round1: {}", round1);
    println!("Round2: {}", round2);
}
