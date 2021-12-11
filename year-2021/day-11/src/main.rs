static INPUT_DATA: &str = include_str!("input.txt");

#[derive(Debug)]
struct Cell {
    pub energy: u64,
    pub is_flashed: bool,
}

fn flash(grid: &mut Vec<Vec<Cell>>, row: usize, col: usize) {
    if !grid[row][col].is_flashed {
        grid[row][col].is_flashed = true;
        grid[row][col].energy = 0;
        if row > 0 && !grid[row - 1][col].is_flashed {
            grid[row - 1][col].energy += 1;
        }
        if col > 0 && !grid[row][col - 1].is_flashed {
            grid[row][col - 1].energy += 1;
        }
        if row < grid.len() - 1 && !grid[row + 1][col].is_flashed {
            grid[row + 1][col].energy += 1;
        }
        if col < grid[0].len() - 1 && !grid[row][col + 1].is_flashed {
            grid[row][col + 1].energy += 1;
        }
        if row > 0 && col > 0 && !grid[row - 1][col - 1].is_flashed {
            grid[row - 1][col - 1].energy += 1;
        }
        if row > 0 && col < grid[0].len() - 1 && !grid[row - 1][col + 1].is_flashed {
            grid[row - 1][col + 1].energy += 1;
        }
        if row < grid.len() - 1 && col > 0 && !grid[row + 1][col - 1].is_flashed {
            grid[row + 1][col - 1].energy += 1;
        }
        if row < grid.len() - 1 && col < grid[0].len() - 1 && !grid[row + 1][col + 1].is_flashed {
            grid[row + 1][col + 1].energy += 1;
        }
    }
}

fn main() {
    let mut grid = INPUT_DATA
        .split("\n")
        .into_iter()
        .map(|str| {
            str.bytes()
                .into_iter()
                .map(|energy| Cell {
                    energy: (energy - 48) as u64,
                    is_flashed: false,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut step = 0;
    let mut round1 = 0;
    let mut round2 = 0;

    'outer: loop {
        step += 1;
        for cell in grid.iter_mut().flatten() {
            cell.energy += 1;
        }
        let mut step_flahes = 0;
        'inner: loop {
            let mut has_flashes = false;
            for row in 0..grid.len() {
                for col in 0..grid[0].len() {
                    if grid[row][col].energy > 9 {
                        flash(&mut grid, row, col);
                        has_flashes = true;
                        step_flahes += 1;
                        if step <= 100 {
                            round1 += 1;
                        }
                    }
                }
            }
            if !has_flashes {
                break 'inner;
            }
        }
        for cell in grid.iter_mut().flatten() {
            cell.is_flashed = false;
        }
        if step_flahes == 100 && round2 == 0 {
            round2 = step;
        }
        if round2 > 0 && step > 100 {
            break 'outer;
        }
    }

    println!("Round 1: {}", round1);
    println!("Round 2: {}", round2);
}
