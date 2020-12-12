use regex::Regex;

static INPUT_DATA: &str = include_str!("input.txt");

fn main() {

    let directions = ['E', 'S', 'W', 'N'];
    let re = Regex::new(r"([NSEWLRF])([0-9]+)").unwrap();
    let commands = INPUT_DATA.split("\n").collect::<Vec<&str>>();

    let mut direction: usize = 0;
    let mut e_pos: i64 = 0;
    let mut n_pos: i64 = 0;

    let mut e_pos2: i64 = 0;
    let mut n_pos2: i64 = 0;
    let mut e_wp: i64 = 10;
    let mut n_wp: i64 = 1;

    for str in commands.iter() {

        let cap = re.captures_iter(str).nth(0).unwrap();
        let cmd = cap[1].chars().nth(0).unwrap();
        let val = cap[2].parse::<i64>().unwrap();

        match cmd {
            'E' => { 
                e_pos += val;
                e_wp += val;
            },
            'W' => { 
                e_pos -= val; 
                e_wp -= val;
            },
            'N' => { 
                n_pos += val; 
                n_wp += val;
            },
            'S' => { 
                n_pos -= val; 
                n_wp -= val;
            },
            'R' => {
                let rot = (val as usize / 90) % 4;
                direction = (direction + rot) % 4;
                match rot {
                    1 => { let tmp = e_wp; e_wp = n_wp; n_wp = -tmp; },
                    2 => { e_wp = -e_wp; n_wp = -n_wp; },
                    3 => { let tmp = e_wp; e_wp = -n_wp; n_wp = tmp; },
                    _ => {}
                }
            },
            'L' => {
                let rot = (val as usize / 90) % 4;
                direction = (direction + (4 - rot)) % 4;
                match rot {
                    1 => { let tmp = e_wp; e_wp = -n_wp; n_wp = tmp; },
                    2 => { e_wp = -e_wp; n_wp = -n_wp; },
                    3 => { let tmp = e_wp; e_wp = n_wp; n_wp = -tmp; },
                    _ => {}
                }
            },
            'F' => {
                match directions[direction] {
                    'E' => { e_pos += val; },
                    'W' => { e_pos -= val; },
                    'N' => { n_pos += val; },
                    'S' => { n_pos -= val; },
                    _ => {}
                }
                e_pos2 += e_wp * val;
                n_pos2 += n_wp * val;
            }
            _ => {}
        };
    }

    println!("{}", e_pos);
    println!("{}", n_pos);
    println!("{}", e_pos.abs() + n_pos.abs());

    println!("{}", e_pos2);
    println!("{}", n_pos2);
    println!("{}", e_pos2.abs() + n_pos2.abs());
}
