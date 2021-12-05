static INPUT_DATA: &str = include_str!("input.txt");

use regex::Regex;
use std::collections::HashMap;

struct Matrix {
    map: HashMap<(u64, u64), u64>,
}

impl Matrix {
    pub fn new() -> Self {
        Self {
            map: HashMap::<(u64, u64), u64>::new(),
        }
    }

    pub fn fill_matrix(&mut self, x1: u64, y1: u64, x2: u64, y2: u64, diag: bool) {
        let keys = if x1 == x2 {
            (if y1 > y2 { y2..=y1 } else { y1..=y2 })
                .map(|y| (x1, y))
                .collect()
        } else if y2 == y1 {
            (if x1 > x2 { x2..=x1 } else { x1..=x2 })
                .map(|x| (x, y1))
                .collect()
        } else if diag {
            let range_y = if y1 > y2 { y1 - y2 } else { y2 - y1 };
            let range_x = if x1 > x2 { x1 - x2 } else { x2 - x1 };
            let range = if range_x >= range_y { range_y } else { range_x };
            (0..=range)
                .into_iter()
                .map(|i| {
                    let x = if x2 >= x1 { x1 + i } else { x1 - i };
                    let y = if y2 >= y1 { y1 + i } else { y1 - i };
                    (x, y)
                })
                .collect()
        } else {
            vec![]
        };

        for key in keys {
            if self.map.contains_key(&key) {
                *self.map.get_mut(&key).unwrap() += 1;
            } else {
                self.map.insert(key, 1);
            }
        }
    }

    pub fn get_overlapping_oount(&self) -> u64 {
        self.map
            .iter()
            .map(|item| if *item.1 > 1 { 1 } else { 0 })
            .sum::<u64>()
    }
}

fn main() {
    let mut matrix1 = Matrix::new();
    let mut matrix2 = Matrix::new();
    let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)$").unwrap();

    for str in INPUT_DATA.split("\n").into_iter() {
        let cap = re.captures(str).unwrap();
        let x1 = cap[1].parse::<u64>().unwrap();
        let y1 = cap[2].parse::<u64>().unwrap();
        let x2 = cap[3].parse::<u64>().unwrap();
        let y2 = cap[4].parse::<u64>().unwrap();
        matrix1.fill_matrix(x1, y1, x2, y2, false);
        matrix2.fill_matrix(x1, y1, x2, y2, true);
    }

    println!("Round 1: {}", matrix1.get_overlapping_oount());
    println!("Round 2: {}", matrix2.get_overlapping_oount());
}
