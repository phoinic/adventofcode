static INPUT_DATA: &str = include_str!("input.txt");

#[derive(Debug, Clone)]
enum Command {
    Noop,
    AddX,
}

#[derive(Debug, Clone)]
struct Instruction {
    cmd: Command,
    attr: Option<i32>,
}

struct ExecutorState {
    cycle: usize,
}

struct ExecutoerRegisters {
    x: i32,
}

#[derive(Debug, Clone)]
struct ExecutorReport {
    signal_strength: Vec<i32>,
    pixels: [char; 240],
}

struct Executor {
    state: ExecutorState,
    registers: ExecutoerRegisters,
    report: ExecutorReport,
}

impl Executor {
    fn new() -> Self {
        Self {
            state: ExecutorState { cycle: 0 },
            registers: ExecutoerRegisters { x: 1 },
            report: ExecutorReport {
                signal_strength: vec![0],
                pixels: ['.'; 240],
            },
        }
    }

    fn run(&mut self, program: Vec<Instruction>) {
        self.state.cycle = 0;
        self.registers.x = 1;
        self.report.signal_strength = vec![0];
        self.report.pixels = ['.'; 240];

        for insturction in program {
            match insturction.cmd {
                Command::Noop => self.cmd_noop(),
                Command::AddX => self.cmd_addx(insturction.attr.unwrap()),
            }
        }
    }

    fn tick(&mut self) {
        let pixel_pos = self.state.cycle;
        let pixel_row_pos = pixel_pos as i32 % 40;

        self.state.cycle += 1;

        self.report
            .signal_strength
            .push(self.state.cycle as i32 * self.registers.x);

        if (self.registers.x - 1..self.registers.x + 2).contains(&pixel_row_pos) {
            self.report.pixels[pixel_pos] = '#';
        } else {
            self.report.pixels[pixel_pos] = '.';
        }
    }

    fn cmd_noop(&mut self) {
        self.tick();
    }

    fn cmd_addx(&mut self, attr: i32) {
        self.tick();
        self.tick();
        self.registers.x += attr;
    }
}

fn read_input() -> Vec<Instruction> {
    let mut program = vec![];

    for line in INPUT_DATA.split("\n") {
        let instruction_parts = line.split(" ").collect::<Vec<_>>();
        program.push(match instruction_parts[0] {
            "noop" => Instruction {
                cmd: Command::Noop,
                attr: None,
            },
            "addx" => Instruction {
                cmd: Command::AddX,
                attr: Some(instruction_parts[1].parse::<i32>().unwrap()),
            },
            _ => panic!("Invalid instruction"),
        });
    }

    program
}

fn main() {
    let program = read_input();

    println!("{:?}", program);

    let mut executor = Executor::new();
    executor.run(program);

    let round1 = executor.report.signal_strength[20]
        + executor.report.signal_strength[60]
        + executor.report.signal_strength[100]
        + executor.report.signal_strength[140]
        + executor.report.signal_strength[180]
        + executor.report.signal_strength[220];

    println!("Round 1: {}", round1);

    println!("Round 2:");
    for (i, pixel) in executor.report.pixels.iter().enumerate() {
        if i > 0 && i % 40 == 0 {
            println!();
        }
        print!("{}", pixel);
    }
    println!();
}
