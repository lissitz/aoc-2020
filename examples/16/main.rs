use std::collections::{HashMap, HashSet};
use std::ops::Range;
fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("examples/16/input.txt")?;
    let mut parts = input.split("\n\n");
    let fields: Vec<_> = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| Field::from(line))
        .collect();
    let my_ticket = parts
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|line| Ticket::from(line))
        .next()
        .unwrap();

    let nearby: Vec<_> = parts
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|line| Ticket::from(line))
        .collect();

    let mut error_rate = 0;
    for ticket in &nearby {
        'a: for n in &ticket.numbers {
            for field in &fields {
                for range in &field.ranges {
                    if range.contains(&(*n as usize)) {
                        continue 'a;
                    }
                }
            }
            error_rate += n;
        }
    }
    println!("{}", error_rate);

    // Part 2
    let valid: Vec<_> = nearby
        .iter()
        .filter(|ticket| {
            'a: for n in &ticket.numbers {
                for field in &fields {
                    for range in &field.ranges {
                        if range.contains(&(*n as usize)) {
                            continue 'a;
                        }
                    }
                }
                return false;
            }
            true
        })
        .collect();

    let mut d: Vec<HashSet<usize>> = (0..valid[0].numbers.len())
        .map(|_| ((0..fields.len()).collect()))
        .collect();
    for ticket in &valid {
        for (i, n) in ticket.numbers.iter().enumerate() {
            for (j, field) in fields.iter().enumerate() {
                let mut in_range = false;
                for range in &field.ranges {
                    if range.contains(&(*n as usize)) {
                        in_range = true;
                        break;
                    }
                }
                if !in_range {
                    d[i].remove(&j);
                }
            }
        }
    }
    let mut d = d.iter().enumerate().collect::<Vec<_>>();
    d.sort_by(|x, y| x.1.len().cmp(&y.1.len()));
    let mut map = HashMap::new();
    let mut found = HashSet::<usize>::new();
    for (i, s) in &d {
        let field = s.difference(&found).next().unwrap().clone();
        map.insert(i, &fields[field]);
        found.insert(field);
    }
    let solution = map.iter()
        .filter(|x| x.1.field.contains("departure"))
        .map(|x| my_ticket.numbers[**x.0])
        .product::<u64>();

    println!("{:?}", solution);

    Ok(())
}

#[derive(Debug)]
struct Field<'a> {
    field: &'a str,
    ranges: Vec<Range<usize>>,
}

impl<'a> From<&'a str> for Field<'a> {
    fn from(s: &'a str) -> Self {
        let mut it = s.split(':').map(|x| x.trim());
        let field = it.next().unwrap();
        let ranges = it.next().unwrap().split("or").map(|x| x.trim());
        let ranges = ranges
            .map(|range| {
                let mut it = range.split('-');
                let start = it.next().unwrap().parse::<usize>().unwrap();
                let end = it.next().unwrap().parse::<usize>().unwrap();
                Range {
                    start,
                    end: end + 1,
                }
            })
            .collect();
        Field { field, ranges }
    }
}

#[derive(Debug)]
struct Ticket {
    numbers: Vec<u64>,
}

impl From<&str> for Ticket {
    fn from(s: &str) -> Self {
        Ticket {
            numbers: s.split(',').map(|x| x.parse::<u64>().unwrap()).collect(),
        }
    }
}
