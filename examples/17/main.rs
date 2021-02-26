use std::clone::Clone;
use std::convert::From;
use std::default::Default;
use std::fmt;
use std::ops::{Index, IndexMut};
fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("examples/17/input.txt")?;
    let data: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.trim().chars().map(|x| Cell::from(x)).collect())
        .collect();
    let n = 50;
    let mut grid: Grid<Cell> = Grid::new(n, n, n);

    // fill with the initial data
    let x_len = data[0].len();
    let z = grid.z / 2;
    let mut y = (grid.y - data.len()) / 2;
    for line in &data {
        let mut x = (grid.x - x_len) / 2;
        for cell in line {
            grid[[x, y, z]] = *cell;
            x += 1;
        }
        y += 1;
    }

    let mut next = grid.clone();
    for _ in 0..6 {
        for z in 0..grid.z {
            for y in 0..grid.y {
                for x in 0..grid.x {
                    let pos = [x, y, z];
                    let neighbors = neighbors(&pos, &grid);
                    match grid[[x, y, z]] {
                        Cell::Active => match neighbors {
                            2 | 3 => {
                                next[pos] = Cell::Active;
                            }
                            _ => {
                                next[pos] = Cell::Inactive;
                            }
                        },
                        Cell::Inactive => match neighbors {
                            3 => {
                                next[pos] = Cell::Active;
                            }
                            _ => {
                                next[pos] = Cell::Inactive;
                            }
                        },
                    }
                }
            }
        }
        std::mem::swap(&mut grid, &mut next);
    }

    let active = grid.data.iter().filter(|x| **x == Cell::Active).count();
    println!("{}", active);

    // Part 2

    let mut grid: Grid4<Cell> = Grid4::new(n, n, n, n);

    // fill with the initial data
    let x_len = data[0].len();
    let z = grid.z / 2;
    let w = grid.z / 2;
    let mut y = (grid.y - data.len()) / 2;
    for line in &data {
        let mut x = (grid.x - x_len) / 2;
        for cell in line {
            grid[[x, y, z, w]] = *cell;
            x += 1;
        }
        y += 1;
    }

    let mut next = grid.clone();
    for _ in 0..6 {
        for w in 0..grid.z {
            for z in 0..grid.z {
                for y in 0..grid.y {
                    for x in 0..grid.x {
                        let pos = [x, y, z, w];
                        let neighbors = neighbors4(&pos, &grid);
                        match grid[[x, y, z, w]] {
                            Cell::Active => match neighbors {
                                2 | 3 => {
                                    next[pos] = Cell::Active;
                                }
                                _ => {
                                    next[pos] = Cell::Inactive;
                                }
                            },
                            Cell::Inactive => match neighbors {
                                3 => {
                                    next[pos] = Cell::Active;
                                }
                                _ => {
                                    next[pos] = Cell::Inactive;
                                }
                            },
                        }
                    }
                }
            }
        }
        std::mem::swap(&mut grid, &mut next);
    }

    let active = grid.data.iter().filter(|x| **x == Cell::Active).count();
    println!("{}", active);

    Ok(())
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '#' => Cell::Active,
            '.' => Cell::Inactive,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Cell {
    Active,
    Inactive,
}

impl Default for Cell {
    fn default() -> Self {
        Cell::Inactive
    }
}

impl From<Cell> for char {
    fn from(c: Cell) -> char {
        match c {
            Cell::Active => '#',
            Cell::Inactive => '.',
        }
    }
}

struct Grid<T> {
    data: Vec<T>,
    x: usize,
    y: usize,
    z: usize,
}

impl<T> Index<[usize; 3]> for Grid<T> {
    type Output = T;
    fn index(&self, index: [usize; 3]) -> &T {
        &self.data[index[0] + index[1] * self.x + index[2] * self.x * self.y]
    }
}

impl<T> IndexMut<[usize; 3]> for Grid<T> {
    fn index_mut(&mut self, index: [usize; 3]) -> &mut T {
        &mut self.data[index[0] + index[1] * self.x + index[2] * self.x * self.y]
    }
}

impl<T> Grid<T>
where
    T: Clone,
    T: Default,
{
    fn new(x: usize, y: usize, z: usize) -> Self {
        Self {
            data: vec![Default::default(); x * y * z],
            x,
            y,
            z,
        }
    }
}

impl<T> Clone for Grid<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

fn neighbors(pos: &[usize; 3], grid: &Grid<Cell>) -> u64 {
    let mut neighbors = 0;
    let dirs = &[-1 as i64, 0, 1];
    for x in dirs {
        for y in dirs {
            for z in dirs {
                if *x == 0 && *y == 0 && *z == 0 {
                    continue;
                }
                let [a, b, c] = pos;
                let i = (x + *a as i64) as usize;
                let j = (y + *b as i64) as usize;
                let k = (z + *c as i64) as usize;
                if i != usize::MAX
                    && j != usize::MAX
                    && k != usize::MAX
                    && i != grid.x
                    && j != grid.y
                    && k != grid.z
                {
                    if let Cell::Active = grid[[i, j, k]] {
                        neighbors += 1;
                    }
                }
            }
        }
    }
    neighbors
}

impl fmt::Display for Grid<Cell> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for z in 0..self.z {
            let start = z * self.x * self.y;
            let has_active = self.data[start..(start + self.x * self.y)]
                .iter()
                .find(|x| **x == Cell::Active);
            if has_active.is_none() {
                continue;
            }
            writeln!(f, "z={}", z)?;
            for y in 0..self.y {
                let start = z * self.x * self.y + y * self.x;
                let s = self.data[start..(start + self.x)]
                    .iter()
                    .map(|x| char::from(*x))
                    .collect::<String>();
                writeln!(f, "{}", s)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

struct Grid4<T> {
    data: Vec<T>,
    x: usize,
    y: usize,
    z: usize,
    w: usize,
}

impl<T> Index<[usize; 4]> for Grid4<T> {
    type Output = T;
    fn index(&self, index: [usize; 4]) -> &T {
        &self.data[index[0]
            + index[1] * self.x
            + index[2] * self.x * self.y
            + index[3] * self.x * self.y * self.z]
    }
}

impl<T> IndexMut<[usize; 4]> for Grid4<T> {
    fn index_mut(&mut self, index: [usize; 4]) -> &mut T {
        &mut self.data[index[0]
            + index[1] * self.x
            + index[2] * self.x * self.y
            + index[3] * self.x * self.y * self.z]
    }
}

impl<T> Grid4<T>
where
    T: Clone,
    T: Default,
{
    fn new(x: usize, y: usize, z: usize, w: usize) -> Self {
        Self {
            data: vec![Default::default(); x * y * z * w],
            x,
            y,
            z,
            w,
        }
    }
}

impl<T> Clone for Grid4<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            x: self.x,
            y: self.y,
            z: self.z,
            w: self.w,
        }
    }
}

fn neighbors4(pos: &[usize; 4], grid: &Grid4<Cell>) -> u64 {
    let mut neighbors = 0;
    let dirs = &[-1 as i64, 0, 1];
    for x in dirs {
        for y in dirs {
            for z in dirs {
                for w in dirs {
                    if *x == 0 && *y == 0 && *z == 0 && *w == 0 {
                        continue;
                    }
                    let [a, b, c, d] = pos;
                    let i = (x + *a as i64) as usize;
                    let j = (y + *b as i64) as usize;
                    let k = (z + *c as i64) as usize;
                    let l = (w + *d as i64) as usize;
                    if i != usize::MAX
                        && j != usize::MAX
                        && k != usize::MAX
                        && l != usize::MAX
                        && i != grid.x
                        && j != grid.y
                        && k != grid.z
                        && l != grid.w
                    {
                        if let Cell::Active = grid[[i, j, k, l]] {
                            neighbors += 1;
                        }
                    }
                }
            }
        }
    }
    neighbors
}
