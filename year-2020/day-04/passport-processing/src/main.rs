use regex::Regex;

static INPUT_DATA: &str = include_str!("input.txt");
struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: String
}

impl Passport {

    pub fn is_valid(&self) -> bool {

        (self.byr.len() > 0)
        && (self.iyr.len() > 0)
        && (self.eyr.len() > 0) 
        && (self.hgt.len() > 0)
        && (self.hcl.len() > 0)
        && (self.ecl.len() > 0)
        && (self.pid.len() > 0)
    }

    pub fn is_valid_data(&self) -> bool {

        let re = Regex::new(r"^[0-9]{4}$").unwrap();

        if !re.is_match(self.byr.as_str()) {
            return false;
        }
        let byr = self.byr.as_str().parse::<u64>().unwrap();
        if byr < 1920 || byr > 2002 {
            return false;
        }

        if !re.is_match(self.iyr.as_str()) {
            return false;
        }
        let iyr = self.iyr.as_str().parse::<u64>().unwrap();
        if iyr < 2010 || iyr > 2020 {
            return false;
        }

        if !re.is_match(self.eyr.as_str()) {
            return false;
        }
        let eyr = self.eyr.as_str().parse::<u64>().unwrap();
        if eyr < 2020 || eyr > 2030 {
            return false;
        }

        let re = Regex::new(r"^([0-9]{2,3})(cm|in)$").unwrap();

        if !re.is_match(self.hgt.as_str()) {
            return false;
        }
        let cap = re.captures_iter(self.hgt.as_str()).nth(0).unwrap();
        let hgt = cap[1].parse::<u64>().unwrap();
        match &cap[2] {
            "cm" => if hgt < 150 || hgt > 193 { return false; },
            "in" => if hgt < 59 || hgt > 76 { return false; },
            _ => {}
        }

        let re = Regex::new(r"^\#[0-9a-f]{6}$").unwrap();

        if !re.is_match(self.hcl.as_str()) {
            return false;
        }

        let re = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();

        if !re.is_match(self.ecl.as_str()) {
            return false;
        }

        let re = Regex::new(r"^[0-9]{9}$").unwrap();

        if !re.is_match(self.pid.as_str()) {
            return false;
        }

        true
    }
}

fn parse_passport(passport_str: String) -> Passport {

    let re = Regex::new(r"([^\s]+):([^\s]+)").unwrap();

    let mut rv = Passport {
        byr: String::from(""),
        iyr: String::from(""),
        eyr: String::from(""),
        hgt: String::from(""),
        hcl: String::from(""),
        ecl: String::from(""),
        pid: String::from(""),
        cid: String::from(""),
    };

    for pair in re.captures_iter(passport_str.as_str()) {
        match &pair[1] {
            "byr" => rv.byr = pair[2].to_string(),
            "iyr" => rv.iyr = pair[2].to_string(),
            "eyr" => rv.eyr = pair[2].to_string(),
            "hcl" => rv.hcl = pair[2].to_string(),
            "hgt" => rv.hgt = pair[2].to_string(),
            "ecl" => rv.ecl = pair[2].to_string(),
            "pid" => rv.pid = pair[2].to_string(),
            "cid" => rv.cid = pair[2].to_string(),
            _ => {}
        }
    }

    rv
}

fn main() {
    
    let mut passports: Vec<Passport> = vec![];
    let mut passport_str = String::from("");

    for str in INPUT_DATA.split("\n") {

        if str == "" {
            passports.push(parse_passport(passport_str));
            passport_str = String::from("");
        } else {
            passport_str = format!("{} {}", passport_str, str);
        }
    }

    passports.push(parse_passport(passport_str));

    let mut valid_count = 0;
    let mut valid_data_count = 0;

    for passport in passports {

        if passport.is_valid() {
            valid_count += 1;

            if passport.is_valid_data() {
                valid_data_count += 1;
            }
        }
    }

    println!("{}", valid_count);
    println!("{}", valid_data_count);
}
