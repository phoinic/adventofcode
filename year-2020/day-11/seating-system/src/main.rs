static INPUT_DATA: &str = include_str!("input.txt");

#[derive(Copy, Clone)]
enum Logic {
    Nearest,
    Seeable,
}

fn dump_matrix(matrix: &Vec<Vec<char>>) {
    for r in matrix {
        for c in r {
            print!("{}", c);
        }
        println!("");
    }
    println!("");
}

fn get_sum_occupied(matrix: &Vec<Vec<char>>) -> u64{

    matrix.iter().fold(0, |sum, row| {
        sum + row.iter().fold(0, |sum, &c| {
            sum + if c == '#' { 1 } else { 0 }
        })
    })
}

fn get_count_occupied_nearest(x: usize, y: usize, matrix: &Vec<Vec<char>>) -> u64 {

    let mut rv = 0;
    let w = matrix[0].len();
    let h = matrix.len();

    if x > 0 {
        if matrix[y][x-1] == '#' {
            rv += 1;
        }
    }
    if x < w - 1 {
        if matrix[y][x+1] == '#' {
            rv += 1;
        }
    }

    if y > 0 {
        if matrix[y-1][x] == '#' {
            rv += 1;
        }
        if x > 0 {
            if matrix[y-1][x-1] == '#' {
                rv += 1;
            }
        }
        if x < w - 1 {
            if matrix[y-1][x+1] == '#' {
                rv += 1;
            }
        }
    }

    if y < h - 1 {
        if matrix[y+1][x] == '#' {
            rv += 1;
        }
        if x > 0 {
            if matrix[y+1][x-1] == '#' {
                rv += 1;
            }
        }
        if x < w - 1 {
            if matrix[y+1][x+1] == '#' {
                rv += 1;
            }
        }
    }

    rv
} 

fn check_direction(dx: i64, dy: i64, sx: i64, sy: i64, matrix: &Vec<Vec<char>>) -> u64 {

    let w = matrix[0].len() as i64;
    let h = matrix.len() as i64;
    let mut x = sx;
    let mut y = sy;

    while x + dx >= 0 && y + dy >= 0 && x + dx <= w - 1 && y + dy <= h - 1 {

        x += dx;
        y += dy;

        match matrix[y as usize][x as usize] {
            'L' => { return 0 },
            '#' => { return 1 },
            _ => {}
        }
    }

    0
}

fn get_count_occupied_seeable(x: usize, y: usize, matrix: &Vec<Vec<char>>) -> u64 {

    let mut rv = 0;
    let w = matrix[0].len();
    let h = matrix.len();

    if x > 0 {
        rv += check_direction(-1, 0, x as i64, y as i64, matrix);
    }
    if x < w - 1 {
        rv += check_direction(1, 0, x as i64, y as i64, matrix);
    }

    if y > 0 {
        rv += check_direction(0, -1, x as i64, y as i64, matrix);
        if x > 0 {
            rv += check_direction(-1, -1, x as i64, y as i64, matrix);
        }
        if x < w - 1 {
            rv += check_direction(1, -1, x as i64, y as i64, matrix);
        }
    }

    if y < h - 1 {
        rv += check_direction(0, 1, x as i64, y as i64, matrix);
        if x > 0 {
            rv += check_direction(-1, 1, x as i64, y as i64, matrix);
        }
        if x < w - 1 {
            rv += check_direction(1, 1, x as i64, y as i64, matrix);
        }
    }

    rv
} 

fn simulate(matrix: &mut Vec<Vec<char>>, logic: Logic) -> u64 {

    let mut rv = 0;
    let w = matrix[0].len();
    let h = matrix.len();

    let mut new_matrix = matrix.clone();

    for y in 0..h {
        for x in 0..w {            
            let count_occupied = match logic {
                Logic::Nearest => get_count_occupied_nearest(x, y, matrix),
                Logic::Seeable => get_count_occupied_seeable(x, y, matrix),
            };
            let limit_occupied = match logic {
                Logic::Nearest => 4,
                Logic::Seeable => 5,
            };
            if matrix[y][x] == 'L' {
                if count_occupied == 0 {
                    new_matrix[y][x] = '#';
                    rv += 1;
                }
            }
            if matrix[y][x] == '#' {
                if count_occupied >= limit_occupied {
                    new_matrix[y][x] = 'L';
                    rv += 1;
                }
            }
        }
    }

    for y in 0..h {
        for x in 0..w {
            matrix[y][x] = new_matrix[y][x]
        }
    }

    rv
}

fn main() {

    let mut matrix1 = INPUT_DATA.split("\n").map(
        |str| {
            str.chars().collect::<Vec<char>>()
        }
    ).collect::<Vec<Vec<char>>>();

    let mut matrix2 = matrix1.clone();

    loop {

        let changes = simulate(&mut matrix1, Logic::Nearest);

        if changes == 0 {
            break;
        }
    }

    dump_matrix(&matrix1);
    println!("{}", get_sum_occupied(&matrix1));
    println!("");

    loop {

        let changes = simulate(&mut matrix2, Logic::Seeable);

        if changes == 0 {
            break;
        }
    }

    dump_matrix(&matrix2);
    println!("{}", get_sum_occupied(&matrix2));
    println!("");
}
