static INPUT_DATA: &str = include_str!("input.txt");

#[derive(Copy, Clone)]
enum OperationType {
    Nop,
    Acc,
    Jmp    
}

#[derive(Copy, Clone)]
struct Operation {
    op_type: OperationType,
    op_param: i64, 
    counter: usize,
}

struct Program {
    acc: i64,
    pos: i64,
    pos_fix: i64,
    operations: Vec<Operation>,    
}

impl Program {
    
    pub fn load(code: &str) -> Program {

        let operations = code.split("\n").map(|line| {

            let parts = line.split(" ").collect::<Vec<&str>>();

            let op_type = match parts[0] {
                "nop" => OperationType::Nop,
                "acc" => OperationType::Acc,
                "jmp" => OperationType::Jmp,
                _ => OperationType::Nop
            };

            let op_param = parts[1].parse::<i64>().unwrap();

            Operation{
                op_type,
                op_param,
                counter: 0
            }

        }).collect::<Vec<Operation>>();

        Program {
            acc: 0,
            pos: 0,
            pos_fix: -1,
            operations: operations
        }
    }

    pub fn run(&mut self) -> (i64, bool) {
        
        self.acc = 0;
        self.pos = 0;

        for i in 0..self.operations.len() {
            self.operations[i].counter = 0;
        }

        loop {

            let pos = self.pos;

            if pos as usize == self.operations.len() {
                return (self.acc, true);
            }

            if pos < 0 || pos as usize > self.operations.len() {
                return (self.acc, false);
            }

            let operation = &self.operations[pos as usize];

            if operation.counter > 0 {
                return (self.acc, false);
            } 

            let op_type = match self.pos_fix {
                pos_fix if self.pos as i64 == pos_fix => {
                    match operation.op_type {
                        OperationType::Nop => OperationType::Jmp,
                        OperationType::Jmp => OperationType::Nop,
                        _ => operation.op_type
                    }
                },
                _ => {
                    operation.op_type
                }
            };

            match op_type {
                OperationType::Acc => {
                    self.acc += operation.op_param;
                    self.pos += 1;
                },
                OperationType::Jmp => {
                    self.pos = self.pos + operation.op_param;
                },
                OperationType::Nop => {
                    self.pos += 1;
                }
            };

            let operation = &mut self.operations[pos as usize];

            operation.counter += 1;            
        }
    }

    pub fn run_fix(&mut self) -> (i64, bool) {

        while self.pos_fix < self.operations.len() as i64 {
            self.pos_fix += 1;
            let (res, success) = self.run();
            if success {
                return (res, success);
            }
        } 

        return (0, false);
    }
}

fn main() {
    
    let mut program = Program::load(INPUT_DATA);

    let (res, success) = program.run();

    println!("{}", res);
    println!("{}", success);

    let (res, success) = program.run_fix();

    println!("{}", res);
    println!("{}", success);
}
