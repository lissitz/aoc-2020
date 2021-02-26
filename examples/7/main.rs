use std::collections::{HashMap, HashSet};
fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("examples/7/input.txt")?;

    let mut rules: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in input.lines() {
        let mut x = line.split("contain");
        let container = x.next().unwrap().trim_end_matches(" bags ");
        let contained = x.next().unwrap();
        if contained == " no other bags." {
            rules.insert(container, HashSet::new());
            continue;
        }
        let contained = contained.split(',');
        let v = contained.map(|x| Bag::from(x).kind).collect();
        rules.insert(container, v);
    }
    let mut good: HashSet<&str> = HashSet::new();
    good.insert(&"shiny gold");
    let mut len = 1;
    loop {
        for (k, v) in &rules {
            if !good.is_disjoint(&v) {
                good.insert(k);
            }
        }
        if len != good.len() {
            len = good.len();
        } else {
            break;
        }
    }
    println!("{}", good.len() - 1);

    // Part 2
    let mut rules: HashMap<&str, Vec<Bag>> = HashMap::new();
    for line in input.lines() {
        let mut x = line.split("contain");
        let container = x.next().unwrap().trim_end_matches(" bags ");
        let contained = x.next().unwrap();
        if contained == " no other bags." {
            rules.insert(container, Vec::new());
            continue;
        }
        let contained = contained.split(',');
        let v = contained.map(|x| Bag::from(x)).collect();
        rules.insert(container, v);
    }
    let bags = rules.get("shiny gold").unwrap();
    // -1 to remove our starting bag
    let count = count_bags(&rules, bags) - 1;
    println!("{}", count);
    Ok(())
}

fn count_bags(rules: &HashMap<&str, Vec<Bag>>, bags: &Vec<Bag>) -> u32 {
    // count the bag itself
    let mut count = 1;
    for bag in bags {
        count += bag.count * count_bags(rules, rules.get(bag.kind).unwrap());
    }
    count
}

struct Bag<'a> {
    count: u32,
    kind: &'a str,
}

impl<'a> From<&'a str> for Bag<'a> {
    fn from(s: &'a str) -> Self {
        let mut words = s.trim().trim_end_matches(|p| p == '.').splitn(2, ' ');
        let count = words.next().unwrap().parse::<u32>().unwrap();
        let kind = words
            .next()
            .unwrap()
            .trim_end_matches(" bags")
            .trim_end_matches(" bag");
        Bag { count, kind }
    }
}
