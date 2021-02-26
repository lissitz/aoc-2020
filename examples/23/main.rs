use std::iter;
fn main() -> std::io::Result<()> {
    // We use the vec index to store the value of a linked list node
    let input = "583976241";
    let v: Vec<_> = input
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect();
    let mut x: Vec<_> = iter::once((0, Node { next: 0 }))
        .chain(v.iter().enumerate().map(|(i, x)| {
            (
                *x,
                Node {
                    next: v[(i + 1) % v.len()],
                },
            )
        }))
        .collect();
    let min = 1;
    let max = 9;
    let start = x[min].0;
    let end = x[max].0;
    x.sort_by_key(|x| x.0);

    let initial_cups: Vec<_> = x.into_iter().map(|x| x.1).collect();
    let mut cups = initial_cups.clone();
    let n = 100;
    iterate(&mut cups, start, n, min, max);
    let mut result = Vec::new();
    let mut current = cups[1].next;
    for _ in 0..(cups.len() - 2) {
        result.push(current);
        current = cups[current].next;
    }
    println!(
        "{:?}",
        result
            .iter()
            .map(|x| x.to_string())
            .collect::<String>()
            .parse::<usize>()
            .unwrap()
    );

    // Part 2
    let mut cups = initial_cups.clone();
    let initial_max = max + 1;
    cups[end].next = initial_max;
    let max = 1e6 as usize;
    cups.extend((initial_max..=max).map(|x| Node { next: x + 1 }));
    cups[max].next = start;
    let n = 1e7 as usize;
    iterate(&mut cups, start, n, min, max);
    let n1 = cups[1].next;
    let n2 = cups[n1].next;
    println!("{}", n1 * n2);

    Ok(())
}

#[derive(Debug, Copy, Clone)]
struct Node {
    next: usize,
}

fn iterate(cups: &mut Vec<Node>, start: usize, n: usize, min: usize, max: usize) {
    let mut current = start;
    for _ in 0..n {
        let n1 = cups[current].next;
        let n2 = cups[n1].next;
        let n3 = cups[n2].next;
        let s = [n1, n2, n3];

        let mut dest = current;
        let destination = loop {
            dest = if dest == min { max } else { dest - 1 };
            if !s.contains(&dest) {
                break dest;
            }
        };
        cups[current].next = cups[n3].next;
        cups[n3].next = cups[destination].next;
        cups[destination].next = n1;
        current = cups[current].next;
    }
}
