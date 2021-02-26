use std::collections::HashSet;
use std::io::{self, BufRead, BufReader};

fn main() -> Result<(), io::Error> {
    let file = std::fs::File::open("examples/1/input.txt")?;
    let lines = BufReader::new(file).lines();
    let s: HashSet<_> = lines.map(|x| x.unwrap().parse::<i32>().unwrap()).collect();
    for n in &s {
        let m = 2020 - n;
        if s.contains(&m) {
            println!("{}, {}", m, n);
            println!("{}", m * n);
            break;
        }
    }

    'outer: for n in &s {
        for u in &s {
            if n == u {
                continue;
            }
            let m = 2020 - n - u;
            if s.contains(&m) {
                println!("{}, {}, {}", m, n, u);
                println!("{}", m * n * u);
                break 'outer;
            }
        }
    }
    Ok(())
}
