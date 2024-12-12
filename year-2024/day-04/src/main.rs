static INPUT_DATA: &str = include_str!("input.txt");

fn search_xmas_round1(chars: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    let mut count = 0usize;

    let to_right = chars[row].len() > col + 3;
    let to_left = col >= 3;
    let to_bottom = chars.len() > row + 3;
    let to_top = row >= 3;

    if to_bottom {
        if chars[row][col] == 'X'
            && chars[row + 1][col] == 'M'
            && chars[row + 2][col] == 'A'
            && chars[row + 3][col] == 'S'
        {
            count += 1
        }
    }

    if to_top {
        if chars[row][col] == 'X'
            && chars[row - 1][col] == 'M'
            && chars[row - 2][col] == 'A'
            && chars[row - 3][col] == 'S'
        {
            count += 1
        }
    }

    if to_right {
        if chars[row][col] == 'X'
            && chars[row][col + 1] == 'M'
            && chars[row][col + 2] == 'A'
            && chars[row][col + 3] == 'S'
        {
            count += 1
        }
        if to_bottom {
            if chars[row][col] == 'X'
                && chars[row + 1][col + 1] == 'M'
                && chars[row + 2][col + 2] == 'A'
                && chars[row + 3][col + 3] == 'S'
            {
                count += 1
            }
        }
        if to_top {
            if chars[row][col] == 'X'
                && chars[row - 1][col + 1] == 'M'
                && chars[row - 2][col + 2] == 'A'
                && chars[row - 3][col + 3] == 'S'
            {
                count += 1
            }
        }
    }

    if to_left {
        if chars[row][col] == 'X'
            && chars[row][col - 1] == 'M'
            && chars[row][col - 2] == 'A'
            && chars[row][col - 3] == 'S'
        {
            count += 1
        }
        if to_bottom {
            if chars[row][col] == 'X'
                && chars[row + 1][col - 1] == 'M'
                && chars[row + 2][col - 2] == 'A'
                && chars[row + 3][col - 3] == 'S'
            {
                count += 1
            }
        }
        if to_top {
            if chars[row][col] == 'X'
                && chars[row - 1][col - 1] == 'M'
                && chars[row - 2][col - 2] == 'A'
                && chars[row - 3][col - 3] == 'S'
            {
                count += 1
            }
        }
    }

    count
}

fn search_xmas_round2(chars: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    if row == 0 || col == 0 || row == chars.len() - 1 || col == chars[row].len() - 1 {
        return 0;
    }

    if chars[row][col] != 'A' {
        return 0;
    }

    if ((chars[row - 1][col - 1] == 'M' && chars[row + 1][col + 1] == 'S')
        || (chars[row - 1][col - 1] == 'S' && chars[row + 1][col + 1] == 'M'))
        && ((chars[row - 1][col + 1] == 'M' && chars[row + 1][col - 1] == 'S')
            || (chars[row - 1][col + 1] == 'S' && chars[row + 1][col - 1] == 'M'))
    {
        return 1;
    }

    0
}

fn main() {
    let mut round1 = 0;
    let mut round2 = 0;

    let chars = INPUT_DATA
        .split("\n")
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for row in 0..chars.len() {
        for col in 0..chars[0].len() {
            round1 += search_xmas_round1(&chars, row, col);
            round2 += search_xmas_round2(&chars, row, col);
        }
    }

    println!("Round1: {}", round1);
    println!("Round2: {}", round2);
}
