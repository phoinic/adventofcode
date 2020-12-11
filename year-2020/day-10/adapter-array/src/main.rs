static INPUT_DATA: &str = include_str!("input.txt");

fn main() {

    let mut nums = INPUT_DATA.split("\n").map(|v| v.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    nums.sort();

    let mut jolts = 0;
    let mut jolts1 = 0;
    let mut jolts3 = 0;
    let mut variants: u64 = 1;

    let mut group: Vec<i64> = vec![0];

    for num in nums.clone() {

        match num - jolts {
            1 => { 
                jolts1 += 1;
                group.push(num);
            },
            3 => { 
                jolts3 += 1;
                variants *= match group.len() {
                    3 => 2,
                    4 => 4,
                    5 => 7,
                    _ => 1
                };
                group = vec![num];
            },
            _ => {
                panic!("Found!");
            }
        }

        jolts = num;
    }
    variants *= match group.len() {
        3 => 2,
        4 => 4,
        5 => 7,
        _ => 1
    };

    jolts3 += 1;

    println!("{}", jolts1 * jolts3);
    println!("{}", variants);
}
