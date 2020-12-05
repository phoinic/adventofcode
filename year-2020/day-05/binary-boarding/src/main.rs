static INPUT_DATA: &str = include_str!("input.txt");

fn main() {
    let mut nums: Vec<u32> = vec![];

    for str in INPUT_DATA.split("\n") {

        let mut start = 1;
        let mut group_size = 128;
        for ch in str.chars().take(7) {
            group_size /= 2;
            if ch == 'B' {
                start += group_size;
            }
        }       
        let row = start - 1;

        let mut start = 1;
        let mut group_size = 8;
        for ch in str.chars().skip(7) {
            group_size /= 2;
            if ch == 'R' {
                start += group_size;
            }
        }       
        let col = start - 1;

        let num = row * 8 + col;

        nums.push(num);
    }

    println!("{}", nums.iter().max().unwrap());

    nums.sort();

    let mut prev_num = 0;
    for num in nums {
        if prev_num != 0 && prev_num + 1 != num {
            println!("{}", prev_num + 1);
        }
        prev_num = num;
    }
}
