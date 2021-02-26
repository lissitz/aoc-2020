use std::collections::HashMap;
use std::ops::{Index, IndexMut};
fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("examples/24/input.txt")?;
    let tiles: Vec<_> = input.lines().map(|line| from(line)).collect();
    let mut counter = HashMap::new();
    for tile in tiles.iter() {
        *counter.entry(tile).or_insert(false) ^= true;
    }
    println!("{}", counter.iter().filter(|x| *x.1).count());

    // Part 2
    let n = 200;
    let mut grid = Grid::<Cell>::new(n, n);
    for (tile, state) in counter {
        grid[[
            (tile[0] + n as isize / 2) as usize,
            (tile[1] + n as isize / 2) as usize,
        ]] = if state { Cell::Black } else { Cell::White };
    }

    let mut new_grid = grid.clone();
    for _ in 0..100 {
        for x in 0..grid.x {
            for y in 0..grid.y {
                let pos = [x, y];
                let neighbors = neighbors(&pos, &grid);
                let count = neighbors.iter().map(|x| grid[*x]).fold(0, |x, y| {
                    if y == Cell::Black {
                        x + 1
                    } else {
                        x
                    }
                });
                match grid[pos] {
                    Cell::Black => {
                        new_grid[pos] = if count == 0 || count > 2 {
                            Cell::White
                        } else {
                            Cell::Black
                        };
                    }
                    Cell::White => {
                        new_grid[pos] = if count == 2 { Cell::Black } else { Cell::White };
                    }
                }
            }
        }
        std::mem::swap(&mut grid, &mut new_grid);
    }
    println!(
        "{}",
        grid.data.iter().fold(0, |x, y| {
            if *y == Cell::Black {
                x + 1
            } else {
                x
            }
        })
    );

    Ok(())
}

fn from(s: &str) -> [isize; 2] {
    let mut it = s.chars();
    let mut x = 0;
    let mut y = 0;
    while let Some(c) = it.next() {
        match c {
            'e' => x += 1,
            'w' => x -= 1,
            's' => {
                let d = it.next().unwrap();
                match d {
                    'w' => {
                        x -= 1;
                        y -= 1;
                    }
                    'e' => {
                        y -= 1;
                    }
                    _ => panic!(),
                }
            }
            'n' => {
                let d = it.next().unwrap();
                match d {
                    'w' => {
                        y += 1;
                    }
                    'e' => {
                        x += 1;
                        y += 1;
                    }
                    _ => panic!(),
                }
            }
            _ => panic!(),
        };
    }
    [x, y]
}

#[derive(Debug)]
struct Grid<T> {
    data: Vec<T>,
    x: usize,
    y: usize,
}

impl<T> Index<[usize; 2]> for Grid<T> {
    type Output = T;
    fn index(&self, index: [usize; 2]) -> &T {
        &self.data[Grid::<T>::_index(index, self.x, self.y)]
    }
}

impl<T> IndexMut<[usize; 2]> for Grid<T> {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut T {
        &mut self.data[Grid::<T>::_index(index, self.x, self.y)]
    }
}

impl<T> Grid<T> {
    fn new(x: usize, y: usize) -> Self
    where
        T: Clone,
        T: Default,
    {
        Self {
            data: vec![Default::default(); x * y],
            x,
            y,
        }
    }

    fn _index(index: [usize; 2], x: usize, y: usize) -> usize {
        index[0] + x * index[1]
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
        }
    }
}

fn neighbors(pos: &[usize; 2], grid: &Grid<Cell>) -> Vec<[usize; 2]> {
    let dirs = vec![[0, 1], [1, 1], [1, 0], [-1, 0], [-1, -1], [0, -1]];
    let mut neighbors = Vec::new();
    for dir in dirs {
        let x = dir[0];
        let y = dir[1];
        let [a, b] = pos;
        let i = (x + *a as i64) as usize;
        let j = (y + *b as i64) as usize;
        if i != usize::MAX && j != usize::MAX && i != grid.x && j != grid.y {
            neighbors.push([i, j]);
        }
    }
    neighbors
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Black,
    White,
}

impl Default for Cell {
    fn default() -> Self {
        Cell::White
    }
}
