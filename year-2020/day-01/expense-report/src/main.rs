static INPUT_DATA: &str = include_str!("input.txt");

fn main() {
    let data = INPUT_DATA.split("\n")
        .map(|str| {
            str.to_string().parse::<i64>().unwrap()
        });

    'outer1: for num1 in data.clone() {
        for num2 in data.clone() {
            if (num1 + num2) == 2020 {
                println!("{}", num1 * num2);
                break 'outer1;
            }
        }
    }
    
    'outer2: for num1 in data.clone() {
        for num2 in data.clone() {
            for num3 in data.clone() {
                if (num1 + num2 + num3) == 2020 {
                    println!("{}", num1 * num2 * num3);
                    break 'outer2;
                }
            }
        }
    }
}
