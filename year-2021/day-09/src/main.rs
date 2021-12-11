use std::collections::HashMap;

static INPUT_DATA: &str = include_str!("input.txt");

#[derive(Debug)]
struct Cell {
    pub height: u8,
    pub is_low: bool,
    pub in_basin: Option<usize>,
}

fn fill_basin(heightmap: &mut Vec<Vec<Cell>>, basin: usize, row: usize, col: usize) {
    if heightmap[row][col].in_basin.is_none() && heightmap[row][col].height != 9 {
        heightmap[row][col].in_basin = Some(basin);
        if row > 0 {
            fill_basin(heightmap, basin, row - 1, col);
        }
        if row < heightmap.len() - 1 {
            fill_basin(heightmap, basin, row + 1, col);
        }
        if col > 0 {
            fill_basin(heightmap, basin, row, col - 1);
        }
        if col < heightmap[0].len() - 1 {
            fill_basin(heightmap, basin, row, col + 1);
        }
    }
}

fn main() {
    let mut heightmap = INPUT_DATA
        .split("\n")
        .into_iter()
        .map(|str| {
            str.bytes()
                .into_iter()
                .map(|height| Cell {
                    height: height - 48,
                    is_low: false,
                    in_basin: None,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut low_points = vec![];
    let mut risk_level: u64 = 0;

    for row in 0..heightmap.len() {
        for col in 0..heightmap[0].len() {
            let height = heightmap[row][col].height;
            if (row == 0 || heightmap[row - 1][col].height > height)
                && (col == 0 || heightmap[row][col - 1].height > height)
                && (row == heightmap.len() - 1 || heightmap[row + 1][col].height > height)
                && (col == heightmap[0].len() - 1 || heightmap[row][col + 1].height > height)
            {
                heightmap[row][col].is_low = true;
                risk_level += height as u64 + 1;
                low_points.push((row, col));
            }
        }
    }

    println!("Round 1: {}", risk_level);

    for (basin, point) in low_points.iter().enumerate() {
        fill_basin(&mut heightmap, basin, point.0, point.1);
    }

    let mut basins = HashMap::<usize, u64>::new();

    for cell in heightmap.iter().flatten() {
        if let Some(key) = cell.in_basin {
            if !basins.contains_key(&key) {
                basins.insert(key, 0);
            }
            *basins.get_mut(&key).unwrap() += 1;
        }
    }

    let mut basins_list = basins.iter().map(|(_, &val)| val).collect::<Vec<_>>();
    basins_list.sort();

    let len = basins_list.len();
    let round2 = basins_list[len - 1] * basins_list[len - 2] * basins_list[len - 3];

    println!("Round 2: {}", round2);
}
