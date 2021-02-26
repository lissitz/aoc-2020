use std::collections::HashMap;
fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("examples/19/input.txt")?;
    let mut it = input.split("\n\n");

    let mut rules: Rules = it
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut it = line.split(':');
            let index = it.next().unwrap().parse::<usize>().unwrap();
            let list = it.next().unwrap().split('|');
            let x = list
                .map(|subrules| {
                    subrules
                        .trim()
                        .split(' ')
                        .map(|x| match x.parse::<usize>() {
                            Ok(i) => Rule::Index(i),
                            _ => Rule::Char(x.chars().nth(1).unwrap()),
                        })
                        .collect()
                })
                .collect();
            (index, x)
        })
        .collect();

    let count = it
        .clone()
        .next()
        .unwrap()
        .lines()
        .filter(|m| {
            let x = &m.chars().collect::<Vec<_>>()[..];
            matches(x, Rule::Index(0), &rules)
        })
        .count();
    println!("{}", count);

    // Part 2

    *rules.get_mut(&8).unwrap() =
        vec![vec![Rule::Index(42)], vec![Rule::Index(42), Rule::Index(8)]];
    *rules.get_mut(&11).unwrap() = vec![
        vec![Rule::Index(42), Rule::Index(31)],
        vec![Rule::Index(42), Rule::Index(11), Rule::Index(31)],
    ];
    let count = it
        .next()
        .unwrap()
        .lines()
        .filter(|m| {
            let x = &m.chars().collect::<Vec<_>>()[..];
            matches(x, Rule::Index(0), &rules)
        })
        .count();
    println!("{}", count);
    Ok(())
}

type Rules = HashMap<usize, Vec<Vec<Rule>>>;

#[derive(Debug, Copy, Clone)]
enum Rule {
    Index(usize),
    Char(char),
}

fn matches(text: &[char], rule: Rule, rules: &Rules) -> bool {
    matches_inner(text, rule, rules, 0, vec![0])
        .iter()
        .find(|x| text.len() == **x)
        .is_some()
}

fn matches_inner(
    text: &[char],
    rule: Rule,
    rules: &Rules,
    depth: usize,
    indices: Vec<usize>,
) -> Vec<usize> {
    if text.is_empty() {
        return indices.clone();
    };
    // A better approach would be to leave the looping rules for the end and limit the loop length to text.len() - n, where n is the number of chars already matched
    if depth > 20 {
        return vec![];
    }
    match rule {
        Rule::Index(rule_index) => rules[&rule_index]
            .iter()
            .flat_map(|subrules| {
                let mut new_indices = indices.clone();
                for subrule in subrules {
                    new_indices =
                        matches_inner(text, *subrule, rules, depth + 1, new_indices.clone());
                }
                new_indices.clone()
            })
            .collect::<Vec<_>>(),
        Rule::Char(c) => {
            let indices = indices
                .iter()
                .filter_map(|index| {
                    text.get(*index)
                        .and_then(|ch| if c == *ch { Some(index + 1) } else { None })
                })
                .collect();
            return indices;
        }
    }
}
