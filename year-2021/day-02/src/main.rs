static INPUT_DATA: &str = include_str!("input.txt");

struct Submarine {
    horizontal: u64,
    depth: u64,
}

impl Submarine {
    pub fn new(horizontal: u64, depth: u64) -> Self {
        Self { horizontal, depth }
    }

    pub fn run(&mut self, command: &str, value: u64) {
        match command {
            "forward" => self.horizontal += value,
            "up" => {
                if value > self.depth {
                    self.depth = 0
                } else {
                    self.depth -= value
                }
            }
            "down" => self.depth += value,
            _ => panic!("Unknown command!"),
        };
    }

    pub fn get_position(&self) -> u64 {
        self.depth * self.horizontal
    }
}

struct Submarine2 {
    horizontal: i64,
    depth: i64,
    aim: i64,
}

impl Submarine2 {
    pub fn new(horizontal: i64, depth: i64, aim: i64) -> Self {
        Self {
            horizontal,
            depth,
            aim,
        }
    }

    pub fn run(&mut self, command: &str, value: u64) {
        let value = value as i64;
        match command {
            "forward" => {
                let change_depth = value * self.aim;
                self.horizontal += value;
                self.depth += change_depth;
                if self.depth < 0 {
                    self.depth = 0;
                }
            }
            "up" => self.aim -= value,
            "down" => self.aim += value,
            _ => panic!("Unknown command!"),
        };
    }

    pub fn get_position(&self) -> i64 {
        self.depth * self.horizontal
    }
}

fn main() {
    let input = INPUT_DATA
        .split("\n")
        .map(|str| str.to_string())
        .collect::<Vec<_>>();

    let mut ship = Submarine::new(0, 0);
    let mut ship2 = Submarine2::new(0, 0, 0);

    for line in input {
        let mut parts = line.split(" ");
        let command = parts.next().unwrap();
        let value = parts.next().unwrap().parse::<u64>().unwrap();
        ship.run(command, value);
        ship2.run(command, value);
    }

    println!("Round 1: {}", ship.get_position());
    println!("Round 2: {}", ship2.get_position());
}
