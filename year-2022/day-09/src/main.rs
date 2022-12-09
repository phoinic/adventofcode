use std::collections::BTreeMap;

type VisitMap = Vec<BTreeMap<(i32, i32), usize>>;

static INPUT_DATA: &str = include_str!("input.txt");

fn get_new_tail_pos(tail_pos: (i32, i32), head_pos: (i32, i32)) -> (i32, i32) {
    if (tail_pos.0 - head_pos.0).abs() <= 1 && (tail_pos.1 - head_pos.1).abs() <= 1 {
        return tail_pos;
    }
    if tail_pos.0 == head_pos.0 {
        return if tail_pos.1 > head_pos.1 {
            (tail_pos.0, tail_pos.1 - 1)
        } else {
            (tail_pos.0, tail_pos.1 + 1)
        };
    }
    if tail_pos.1 == head_pos.1 {
        return if tail_pos.0 > head_pos.0 {
            (tail_pos.0 - 1, tail_pos.1)
        } else {
            (tail_pos.0 + 1, tail_pos.1)
        };
    }
    (
        if tail_pos.0 > head_pos.0 {
            tail_pos.0 - 1
        } else {
            tail_pos.0 + 1
        },
        if tail_pos.1 > head_pos.1 {
            tail_pos.1 - 1
        } else {
            tail_pos.1 + 1
        },
    )
}

fn read_input() -> VisitMap {
    let mut visit_map = vec![BTreeMap::new(); 10];
    let mut knots_pos = vec![(0, 0); 10];

    for line in INPUT_DATA.split("\n") {
        let pair = line.split(" ").collect::<Vec<_>>();
        let steps = pair[1].parse::<usize>().unwrap();
        for _ in 0..steps {
            match pair[0] {
                "U" => knots_pos[0].1 += 1,
                "D" => knots_pos[0].1 -= 1,
                "R" => knots_pos[0].0 += 1,
                "L" => knots_pos[0].0 -= 1,
                _ => {
                    panic!("Unexpected turn: {:?}", pair);
                }
            }
            for i in 1..10 {
                knots_pos[i] = get_new_tail_pos(knots_pos[i], knots_pos[i - 1]);
                if !visit_map[i].contains_key(&knots_pos[i]) {
                    visit_map[i].insert(knots_pos[i], 0);
                }
                *visit_map[i].get_mut(&knots_pos[i]).unwrap() += 1;
            }
        }
    }

    visit_map
}

fn main() {
    let visit_map = read_input();

    println!("{:?}", visit_map);

    let round1 = visit_map[1].len();
    let round2 = visit_map[9].len();

    println!("Round 1: {}", round1);
    println!("Round 2: {}", round2);
}
