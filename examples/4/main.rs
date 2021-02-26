#[macro_use]
extern crate lazy_static;
use regex::Regex;

fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("examples/4/input.txt")?;
    let raw_passports = input.split("\n\n");
    let valid = raw_passports.filter(|x| is_valid_1(x)).count();
    println!("{}", valid);

    // Part 2
    let raw_passports = input.split("\n\n");
    let valid = raw_passports.filter(|x| is_valid_2(x) && is_valid_1(x)).count();
    println!("{}", valid);
    Ok(())
}

fn is_valid_1(s: &str) -> bool {
    for key in &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"] {
        if !s.contains(key) {
            return false;
        }
    }
    true
}

fn is_valid_2(s: &str) -> bool {
    for x in s.split_whitespace() {
        let mut pair = x.split(":");
        let k = pair.next();
        if k.is_none() {
            return false;
        };
        let k = k.unwrap();
        let v = pair.next();
        if v.is_none() {
            return false;
        };
        let v = v.unwrap();
        let valid = match k {
            "byr" => match v.parse::<u32>() {
                Ok(x) if x >= 1920 && x <= 2002 => true,
                _ => false,
            },
            "iyr" => match v.parse::<u32>() {
                Ok(x) if x >= 2010 && x <= 2020 => true,
                _ => false,
            },
            "eyr" => match v.parse::<u32>() {
                Ok(x) if x >= 2020 && x <= 2030 => true,
                _ => false,
            },
            "hgt" => {
                lazy_static! {
                    static ref RE: Regex = Regex::new(r"^([0-9]+)(cm|in)$").unwrap();
                };
                match RE.captures(v) {
                    Some(cap) => match &cap[2] {
                        "cm" => match cap[1].parse::<u32>() {
                            Ok(n) if n >= 150 && n <= 193 => true,
                            _ => false,
                        },
                        "in" => match cap[1].parse::<u32>() {
                            Ok(n) if n >= 59 && n <= 76 => true,
                            _ => false,
                        },
                        _ => false,
                    },
                    None => false,
                }
            }
            "hcl" => {
                lazy_static! {
                    static ref RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
                }
                RE.is_match(v)
            }
            "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&v),
            "pid" => {
                lazy_static! {
                    static ref RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
                }
                RE.is_match(v)
            }
            "cid" => true,
            _ => false,
        };
        if !valid {
            return false;
        }
    }
    true
}
