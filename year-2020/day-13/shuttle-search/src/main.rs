static INPUT_DATA: &str = include_str!("input.txt");

fn main() {

    let lines = INPUT_DATA.split("\n").collect::<Vec<&str>>();
    
    let a_tstamp = lines[0].parse::<u64>().unwrap();
    let buses = lines[1].split(",").filter(|&v| v != "x").map(|v| v.parse::<u64>().unwrap());

    let mut min_mins = a_tstamp;
    let mut min_bus = 0;

    for bus in buses {
        
        let d_tstamp = (a_tstamp as f64 / bus as f64).ceil() as u64 * bus;

        if d_tstamp - a_tstamp < min_mins {
            min_mins = d_tstamp - a_tstamp;
            min_bus = bus;
        }
    }

    println!("{}", min_bus * min_mins);
}
