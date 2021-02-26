use regex::Regex;
use std::io::{self, BufRead, BufReader};
use std::ops::Range;
use std::str::FromStr;

fn main() -> Result<(), io::Error> {
    let file = std::fs::File::open("examples/2/input.txt")?;
    let lines = BufReader::new(file).lines();
    let fields: Vec<Field> = lines.map(|x| x.unwrap().parse().unwrap()).collect();

    // part 1
    let mut c = 0;
    for field in &fields {
        let count =
            field
                .password
                .chars()
                .fold(0, |acc, x| if x == field.policy.ch { acc + 1 } else { acc });
        if field.policy.range.contains(&(count as u32)) {
            c += 1;
        }
    }
    println!("{}", c);

    // part 2
    let c = fields.iter().fold(0, |acc, field| {
        let chars: Vec<_> = field.password.chars().collect();
        if (chars[(field.policy.range.start - 1) as usize] == field.policy.ch)
            ^ (chars[(field.policy.range.end - 2) as usize] == field.policy.ch)
        {
            acc + 1
        } else {
            acc
        }
    });

    println!("{}", c);
    Ok(())
}

struct Field {
    policy: Policy,
    password: String,
}

struct Policy {
    range: Range<u32>,
    ch: char,
}

#[derive(Debug)]
struct ParseFieldError;
impl FromStr for Field {
    type Err = ParseFieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^(\d+)-(\d+) ([a-zA-Z]): (\w+)$").unwrap();
        let cap = re.captures(s).unwrap();
        Ok(Field {
            policy: Policy {
                range: Range {
                    start: cap[1].parse().unwrap(),
                    end: cap[2].parse::<u32>().unwrap() + 1,
                },
                ch: cap[3].parse().unwrap(),
            },
            password: cap[4].parse().unwrap(),
        })
    }
}
