use itertools::Itertools;
use std::clone::Clone;
use std::collections::HashMap;
use std::convert::From;
use std::default::Default;
use std::fmt;
use std::ops::{Index, IndexMut};
fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("examples/20/input.txt")?;
    let tiles: HashMap<usize, Tile> = input
        .split("\n\n")
        .map(|x| {
            let mut it = x.lines();
            let s = it.next().unwrap();
            let start = s.find(' ').unwrap() + 1;
            let end = s.find(':').unwrap();
            let n = s[start..end].parse::<usize>().unwrap();

            let data: Vec<Vec<_>> = it
                .map(|line| line.trim().chars().map(|x| Cell::from(x)).collect())
                .collect();

            let x_len = data[0].len();
            let y_len = data.len();
            let mut tile: Grid<Cell> = Grid::new(x_len, y_len);
            for (y, line) in data.iter().enumerate() {
                for (x, cell) in line.iter().enumerate() {
                    tile[[x, y]] = *cell;
                }
            }
            (n, tile)
        })
        .collect();

    let n = (tiles.len() as f64).sqrt() as usize;
    let v: Vec<_> = tiles.iter().map(|(k, _)| k).collect();

    'outer: for (i, mut tile_order) in v.iter().permutations(v.len()).enumerate() {
        let mut photo_ids: Grid<usize> = Grid::new(n, n);
        while let Some(id) = tile_order.pop() {
            photo_ids.data.push(**id);
            let tile = tiles.get(id).unwrap();
            let mut matched = false;
            for placement in tile_placements(&tile) {
                if matches(i, &placement, &photo_ids, &tiles) {
                    photo_ids.data.push(**id);
                    matched = true;
                    break;
                }
            }
            if !matched {
                continue 'outer;
            }
        }
        println!("{:?}", photo_ids);
        break;
    }

    Ok(())
}

fn matches(
    i: usize,
    placement: &TilePlacement,
    photo_ids: &Grid<usize>,
    tiles: &HashMap<usize, Tile>,
) -> bool {
    let x = i % photo_ids.x;
    let y = i / photo_ids.y;
    let pos = [x, y];
    for (dir, neighbor) in neighbors_photo(&pos, &photo_ids) {
        if let Some(id) = photo_ids.get(neighbor) {
            dbg!(&id);
            dbg!(&photo_ids);
            if !can_join(placement, tiles.get(&id).unwrap(), dir) {
                return false;
            }
        }
    }
    return true;
}

fn tile_placements(t: &Tile) -> Vec<TilePlacement> {
    [Angle::A0, Angle::A90, Angle::A180, Angle::A270]
        .iter()
        .flat_map(|angle| {
            [true, false].iter().map(move |flip| TilePlacement {
                t,
                flip: *flip,
                angle: *angle,
            })
        })
        .collect()
}

fn can_join(a: &TilePlacement, b: &Tile, dir: Direction) -> bool {
    match dir {
        Direction::Up => {
            let up = (0..a.t.x).map(|x| a[[x, 0]]);
            let down = (0..b.x).map(|x| b[[x, 0]]);
            up.zip(down).all_equal()
        }
        Direction::Left => {
            let left = (0..a.t.x).map(|x| a[[x, 0]]);
            let right = (0..b.x).map(|x| b[[x, 0]]);
            left.zip(right).all_equal()
        }
    }
}

type Tile = Grid<Cell>;

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '#' => Cell::Black,
            '.' => Cell::White,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Cell {
    Black,
    White,
}

impl Default for Cell {
    fn default() -> Self {
        Cell::White
    }
}

impl From<Cell> for char {
    fn from(c: Cell) -> char {
        match c {
            Cell::Black => '#',
            Cell::White => '.',
        }
    }
}

struct TilePlacement<'a> {
    t: &'a Tile,
    angle: Angle,
    flip: bool,
}

impl<'a> TilePlacement<'a> {
    fn _index(index: [usize; 2], x: usize, y: usize, angle: Angle, flip: bool) -> [usize; 2] {
        let mut a = index[0];
        let mut b = index[1];
        if flip {
            a = x - a;
        }
        match angle {
            Angle::A0 => {}
            Angle::A90 => {
                let tmp = a;
                a = b;
                b = y - tmp;
            }
            Angle::A180 => {
                a = x - a;
                b = y - b;
            }
            Angle::A270 => {
                let tmp = a;
                a = x - b;
                b = tmp;
            }
        }
        [a, b]
    }
}

impl<'a> Index<[usize; 2]> for TilePlacement<'a> {
    type Output = Cell;
    fn index(&self, index: [usize; 2]) -> &Cell {
        &self.t[TilePlacement::_index(index, self.t.x, self.t.y, self.angle, self.flip)]
    }
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

    fn get(&self, index: [usize; 2]) -> Option<&T> {
        self.data.get(Grid::<T>::_index(index, self.x, self.y))
    }

    fn _index(index: [usize; 2], x: usize, y: usize) -> usize {
        let a = index[0];
        let b = index[1];
        a + b * x
    }
}

#[derive(Debug, Copy, Clone)]
enum Angle {
    A0,
    A90,
    A180,
    A270,
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

impl fmt::Display for Grid<Cell> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.y {
            let start = y * self.x;
            let s = self.data[start..(start + self.x)]
                .iter()
                .map(|x| char::from(*x))
                .collect::<String>();
            writeln!(f, "{}", s)?;
        }
        writeln!(f, "")?;
        Ok(())
    }
}

fn neighbors_photo(pos: &[usize; 2], grid: &Grid<usize>) -> Vec<(Direction, [usize; 2])> {
    let dirs = vec![(Direction::Up, [0, 1]), (Direction::Left, [-1, 0])];
    let mut neighbors = Vec::new();
    for dir in dirs {
        let x = dir.1[0];
        let y = dir.1[1];
        let [a, b] = pos;
        let i = (x + *a as i64) as usize;
        let j = (y + *b as i64) as usize;
        if i != usize::MAX && j != usize::MAX && i != grid.x && j != grid.y {
            neighbors.push((dir.0, [i, j]))
        }
    }
    neighbors
}

enum Direction {
    Up,
    Left,
}
