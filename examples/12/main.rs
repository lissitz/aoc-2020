fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("examples/12/input.txt")?;
    let instructions: Vec<_> = input.lines().map(|x| x.trim()).collect();
    let mut x = 0;
    let mut y = 0;
    let mut direction: i64 = 0;
    use Directions::*;
    let directions = [E, N, W, S];
    for instruction in &instructions {
        let (code, n) = instruction.split_at(1);
        let n = n.parse::<i64>().unwrap();
        match code {
            "N" => {
                y += n;
            }
            "S" => {
                y -= n;
            }
            "E" => {
                x += n;
            }
            "W" => {
                x -= n;
            }
            "L" => {
                direction = (direction + n / 90 + 4) % 4;
            }
            "R" => {
                direction = (direction - n / 90 + 4) % 4;
            }
            "F" => match directions[direction as usize] {
                N => y += n,
                S => y -= n,
                E => x += n,
                W => x -= n,
            },
            _ => panic!(),
        }
    }
    println!("{}", x.abs() + y.abs());

    // Part 2
    let mut x = 0;
    let mut y = 0;
    let mut w_x = 10;
    let mut w_y = 1;
    for instruction in &instructions {
        let (code, n) = instruction.split_at(1);
        let n = n.parse::<i64>().unwrap();
        match code {
            "N" => {
                w_y += n;
            }
            "S" => {
                w_y -= n;
            }
            "E" => {
                w_x += n;
            }
            "W" => {
                w_x -= n;
            }
            "L" => match n {
                90 => {
                    let tmp = w_y;
                    w_y = w_x;
                    w_x = -tmp;
                }
                180 => {
                    w_y = -w_y;
                    w_x = -w_x;
                }
                270 => {
                    let tmp = w_y;
                    w_y = -w_x;
                    w_x = tmp;
                }
                _ => panic!(),
            },
            "R" => match n {
                270 => {
                    let tmp = w_y;
                    w_y = w_x;
                    w_x = -tmp;
                }
                180 => {
                    w_y = -w_y;
                    w_x = -w_x;
                }
                90 => {
                    let tmp = w_y;
                    w_y = -w_x;
                    w_x = tmp;
                }
                _ => panic!(),
            },
            "F" => {
                x += n * w_x;
                y += n * w_y;
            }
            _ => panic!(),
        }
    }
    println!("{}", x.abs() + y.abs());
    Ok(())
}

enum Directions {
    N,
    S,
    E,
    W,
}
