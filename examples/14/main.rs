use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;
fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("examples/14/input.txt")?;
    let lines = input.lines().map(|line| line.parse::<Line>().unwrap());
    let mut mask = vec![];
    let mut memory: HashMap<usize, u64> = HashMap::new();
    for line in lines {
        match line {
            Line::Mask(v) => {
                mask = v;
            }
            Line::Write(w) => {
                let mut value = w.value;
                for bit in &mask {
                    match bit {
                        (i, Some(true)) => {
                            value |= 1 << i;
                        }
                        (i, Some(false)) => {
                            value &= !(1 << i);
                        }
                        _ => {}
                    }
                }
                memory.insert(w.address, value);
            }
        }
    }
    println!("{}", memory.iter().map(|(_, v)| v).sum::<u64>());

    // Part 2

    let lines = input.lines().map(|line| line.parse::<Line>().unwrap());
    let mut mask = vec![];
    let mut memory: HashMap<usize, u64> = HashMap::new();
    for line in lines {
        match line {
            Line::Mask(v) => {
                mask = v;
            }
            Line::Write(w) => {
                let mut address = w.address;
                for bit in &mask {
                    match bit {
                        (i, Some(true)) => {
                            address |= 1 << i;
                        }
                        _ => {}
                    }
                }
                let floating = mask.iter().filter_map(|x| match x {
                    (i, None) => Some(i),
                    _ => None,
                });
                let bits_it = floating
                    .map(|x| vec![(*x, true), (*x, false)])
                    .multi_cartesian_product();

                for bits in bits_it {
                    let mut _address = address;
                    for bit in bits {
                        match bit {
                            (i, true) => {
                                _address |= 1 << i;
                            }
                            (i, false) => {
                                _address &= !(1 << i);
                            }
                        }
                    }
                    memory.insert(_address, w.value);
                }
            }
        }
    }

    println!("{}", memory.iter().map(|(_, v)| v).sum::<u64>());
    Ok(())
}

struct Write {
    address: usize,
    value: u64,
}

type Mask = Vec<(usize, Option<bool>)>;
enum Line {
    Mask(Mask),
    Write(Write),
}

#[derive(Debug)]
struct ParseError;
impl FromStr for Line {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split('=').map(|x| x.trim());
        let a = it.next().unwrap();
        match a.chars().nth(1).unwrap() {
            'a' => {
                let mask = it.next().unwrap();
                let mask: Vec<_> = mask
                    .chars()
                    .enumerate()
                    .filter_map(|(i, x)| match x {
                        'X' => Some((35 - i, None)),
                        '1' => Some((35 - i, Some(true))),
                        '0' => Some((35 - i, Some(false))),
                        _ => panic!(),
                    })
                    .collect();
                Ok(Line::Mask(mask))
            }
            'e' => {
                let chs = a.chars().skip(4);
                let address = chs
                    .take_while(|x| *x != ']')
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap();
                let value = it.next().unwrap().parse::<u64>().unwrap();
                Ok(Line::Write(Write { value, address }))
            }
            _ => panic!(),
        }
    }
}
