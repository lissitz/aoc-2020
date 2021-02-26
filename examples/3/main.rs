use std::io::{self, BufRead, BufReader};

fn main() -> Result<(), io::Error> {
    let file = std::fs::File::open("examples/3/input.txt")?;
    let lines = BufReader::new(file).lines();
    let map: Vec<Vec<char>> = lines.map(|x| x.unwrap().chars().collect()).collect();
    let mut x = 0;
    let mut count = 0;
    let l = map[0].len();
    for row in &map {
        if row[(x + l) % l] == '#' {
            count += 1;
        }
        x += 3;
    }
    println!("Part 1:{}", count);

    // Part 2
    let slopes: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut result = 1;
    for (x, y) in slopes.iter() {
        let mut _x = 0;
        let mut count = 0;
        for row in map.iter().step_by(*y) {
            if row[(_x + l) % l] == '#' {
                count += 1;
            }
            _x += x;
        }
        result *= count;
    }
    println!("Part 2:{}", result);
    Ok(())
}
