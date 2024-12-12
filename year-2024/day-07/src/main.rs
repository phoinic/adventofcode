static INPUT_DATA: &str = include_str!("input.txt");

fn check_equation(
    test_value: u128,
    values: &Vec<u128>,
    possible_operators: &Vec<char>,
    pos: usize,
    operators: Vec<char>,
) -> bool {
    let mut result = values[0];

    for i in 0..operators.len() {
        match operators[i] {
            '+' => result += values[i + 1],
            '*' => result *= values[i + 1],
            '|' => {
                let mut result_str = result.to_string();
                result_str.push_str(values[i + 1].to_string().as_str());
                result = str::parse::<u128>(result_str.as_str()).unwrap();
            }
            _ => panic!("Unknown operator"),
        };
    }

    if result > test_value {
        return false;
    }

    if operators.len() == values.len() - 1 {
        return result == test_value;
    }

    possible_operators.iter().any(|&operator| {
        check_equation(
            test_value,
            values,
            possible_operators,
            pos + 1,
            [operators.clone(), vec![operator]].concat(),
        )
    })
}

fn main() {
    let mut round1 = 0;
    let mut round2 = 0;

    let equations = INPUT_DATA
        .split("\n")
        .map(|str| {
            let parts = str.split(": ").collect::<Vec<_>>();
            let test_value = str::parse::<u128>(parts[0]).unwrap();
            let values = parts[1]
                .split_whitespace()
                .map(|o| str::parse::<u128>(o).unwrap())
                .collect::<Vec<_>>();

            (test_value, values)
        })
        .collect::<Vec<_>>();

    for equation in equations {
        if check_equation(equation.0, &equation.1, &vec!['+', '*'], 0, vec![]) {
            round1 += equation.0;
        }
        if check_equation(equation.0, &equation.1, &vec!['+', '*', '|'], 0, vec![]) {
            round2 += equation.0;
        }
    }

    println!("Round1: {}", round1);
    println!("Round2: {}", round2);
}
