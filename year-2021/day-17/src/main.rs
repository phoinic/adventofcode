fn calculate_path(x1: i64, y1: i64, x2: i64, y2: i64, vx: i64, vy: i64) -> (bool, i64) {
    let mut px = 0;
    let mut py = 0;
    let mut max_y = 0;

    let mut vx = vx;
    let mut vy = vy;

    loop {
        px += vx;
        py += vy;

        if py > max_y {
            max_y = py;
        }

        if px > x2 || py < y1 {
            return (false, max_y);
        }

        if px >= x1 && py >= y1 && px <= x2 && py <= y2 {
            return (true, max_y);
        }

        if vx > 0 {
            vx -= 1;
        }

        vy -= 1;
    }
}

fn main() {
    let x1: i64 = 169;
    let x2: i64 = 206;
    let y1: i64 = -108;
    let y2: i64 = -68;

    let mut round1 = 0;
    let mut round2 = 0;

    for vx in 1..x2*2 {
        for vy in -y1.abs()..y1.abs() {
            let res = calculate_path(x1, y1, x2, y2, vx, vy);
            if res.0 {
                round2 += 1;
                if res.1 > round1 {
                    round1 = res.1;
                }
            }
        }
    }

    println!("Round 1: {}", round1);
    println!("Round 2: {}", round2);
}
