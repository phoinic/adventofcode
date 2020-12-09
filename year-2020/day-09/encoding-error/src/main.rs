static INPUT_DATA: &str = include_str!("input.txt");

fn main() {

    let nums = INPUT_DATA.split("\n").map(|v| v.parse::<i64>().unwrap()).collect::<Vec<i64>>();

    let mut res1 = 0;
    let mut res2 = 0;

    for i in 25..nums.len() {
        let num = nums[i];
        let mut found = false;
        'outer: for k1 in (i-25)..i {
            for k2 in (i-25)..i {
                if nums[k1] == nums[k2] {
                    continue;
                }
                if nums[k1] + nums[k2] == num {
                    found = true;
                    break 'outer;
                }
            }                
        }
        if !found {
            res1 = num;
            break;
        }
    }

    println!("{}", res1);

    for i in 0..nums.len() {
        let mut k = i;
        let mut sum = 0;
        let mut min = nums[i];
        let mut max = nums[i];
        while k < nums.len() && sum < res1 {
            sum += nums[k];
            if min > nums[k] {
                min = nums[k];
            }
            if max < nums[k] {
                max = nums[k];
            }
            k += 1;
        }
        if sum == res1 {
            res2 = min + max;
            break;
        }
    }

    println!("{}", res2);
}
