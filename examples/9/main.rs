use std::collections::HashSet;
use std::io;

fn main() -> Result<(), io::Error> {
    let input = std::fs::read_to_string("examples/9/input.txt")?;
    let n = 25;
    let numbers: Vec<_> = input.lines().map(|x| x.parse::<u64>().unwrap()).collect();
    let mut current_numbers: HashSet<u64> = numbers.iter().cloned().collect();
    let mut invalid = 0;
    for (i, x) in numbers.iter().enumerate().skip(n) {
        if !is_pair_sum(*x, &current_numbers) {
            invalid = *x;
            println!("{}", invalid);
            break;
        }
        current_numbers.remove(&numbers[i - n]);
        current_numbers.insert(*x);
    }

    // Part 2

    'outer: for start in 0..numbers.len() {
        for end in (start + 1)..numbers.len() {
            if numbers[start..end].iter().sum::<u64>() == invalid {
                println!(
                    "{}",
                    numbers[start..end].iter().min().unwrap()
                        + numbers[start..end].iter().max().unwrap()
                );
                break 'outer;
            }
        }
    }
    Ok(())
}

fn is_pair_sum(x: u64, current_numbers: &HashSet<u64>) -> bool {
    for a in current_numbers {
        if x > *a && current_numbers.contains(&(x - a)) {
            return true;
        }
    }
    false
}
