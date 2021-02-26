fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("examples/11/input.txt")?;
    let mut grid: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.trim().chars().map(|c| Cell::from(c)).collect())
        .collect();

    let len_x = grid.len();
    let len_y = grid[0].len();

    let directions: Vec<(usize, usize)> = vec![
        (0, 0),
        (0, 1),
        (1, 0),
        (1, 2),
        (2, 1),
        (2, 2),
        (0, 2),
        (2, 0),
    ];

    let mut new_grid: Vec<Vec<_>> = grid.iter().map(|x| x.clone()).collect();
    loop {
        let mut changed = false;
        for x in 0..len_x {
            for y in 0..len_y {
                let cell = grid[x][y];
                if cell == Cell::Floor {
                    continue;
                }
                let mut neighbors = 0;
                for (i, j) in &directions {
                    let a = (x + i).wrapping_sub(1);
                    let b = (y + j).wrapping_sub(1);
                    if a != usize::MAX && a < len_x && b != usize::MAX && b < len_y {
                        if grid[a][b] == Cell::Occupied {
                            neighbors += 1;
                        }
                    }
                }
                match cell {
                    Cell::Occupied => {
                        if neighbors >= 4 {
                            new_grid[x][y] = Cell::Empty;
                            changed = true;
                        } else {
                            new_grid[x][y] = Cell::Occupied;
                        }
                    }
                    Cell::Empty => {
                        if neighbors == 0 {
                            new_grid[x][y] = Cell::Occupied;
                            changed = true;
                        } else {
                            new_grid[x][y] = Cell::Empty;
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
        std::mem::swap(&mut grid, &mut new_grid);

        if !changed {
            break;
        }
    }
    let occupied = grid
        .iter()
        .flat_map(|x| x.iter().filter(|y| **y == Cell::Occupied))
        .count();

    println!("{}", occupied);

    // Part 2
    let mut grid: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.trim().chars().map(|c| Cell::from(c)).collect())
        .collect();
    let directions: Vec<_> = directions
        .iter()
        .map(|(x, y)| (*x as i64 - 1, *y as i64 - 1))
        .collect();

    loop {
        let mut changed = false;
        for x in 0..len_x {
            for y in 0..len_y {
                let cell = grid[x][y];
                if cell == Cell::Floor {
                    continue;
                }
                let mut neighbors = 0;
                for (i, j) in directions.clone() {
                    let mut a = x as i64;
                    let mut b = y as i64;
                    loop {
                        a += i;
                        b += j;
                        if a == -1 || a == len_x as i64 || b == -1 || b == len_y as i64 {
                            break;
                        }
                        match grid[a as usize][b as usize] {
                            Cell::Occupied => {
                                neighbors += 1;
                                break;
                            }
                            Cell::Empty => break,
                            _ => {}
                        }
                    }
                }
                match cell {
                    Cell::Occupied => {
                        if neighbors >= 5 {
                            new_grid[x][y] = Cell::Empty;
                            changed = true;
                        } else {
                            new_grid[x][y] = Cell::Occupied;
                        }
                    }
                    Cell::Empty => {
                        if neighbors == 0 {
                            new_grid[x][y] = Cell::Occupied;
                            changed = true;
                        } else {
                            new_grid[x][y] = Cell::Empty;
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
        std::mem::swap(&mut grid, &mut new_grid);

        if !changed {
            break;
        }
    }
    let occupied = grid
        .iter()
        .flat_map(|x| x.iter().filter(|y| **y == Cell::Occupied))
        .count();

    println!("{}", occupied);

    Ok(())
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum Cell {
    Floor,
    Empty,
    Occupied,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            'L' => Cell::Empty,
            '#' => Cell::Occupied,
            '.' => Cell::Floor,
            _ => unreachable!(),
        }
    }
}