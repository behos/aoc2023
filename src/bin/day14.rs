mod util;

use std::{
    collections::{BTreeMap, BTreeSet},
    fs::read_to_string,
    str::FromStr,
};

use anyhow::{Context, Error, Result};
use util::direction::Direction;

fn main() -> Result<()> {
    let raw = read_to_string("inputs/14.txt").context("Should have been able to read the file")?;
    let raw = raw.trim();
    let platform = Platform::from_str(raw).context("failed to parse platform")?;
    // println!("part 1: {pt1}");
    // println!("part 2: {pt2}");
    Ok(())
}

#[derive(Default)]
struct BidiMap {
    x: BTreeMap<usize, BTreeSet<usize>>,
    y: BTreeMap<usize, BTreeSet<usize>>,
    w: usize,
    h: usize,
}

impl BidiMap {
    fn insert(&mut self, x: usize, y: usize) {
        self.x.entry(x).or_default().insert(y);
        self.y.entry(y).or_default().insert(x);
    }

    fn remove(&mut self, x: usize, y: usize) {
        self.x.entry(x).or_default().remove(&y);
        self.y.entry(y).or_default().remove(&x);
    }

    fn iter_by_x<'a>(&'a self) -> impl DoubleEndedIterator<Item = (usize, usize)> + 'a {
        self.x
            .iter()
            .flat_map(|(x, ys)| ys.iter().map(|y| (*x, *y)))
    }

    fn iter_by_y<'a>(&'a self) -> impl DoubleEndedIterator<Item = (usize, usize)> + 'a {
        self.y
            .iter()
            .flat_map(|(y, xs)| xs.iter().map(|x| (*x, *y)))
    }

    fn first_after(&self, x: usize, y: usize, direction: Direction) -> Option<(usize, usize)> {
        match direction {
            Direction::N => {
                self.x.get(&x).and_then(|e| e.range(0..y).rev().next().map(|y| (x, *y)))
            },
            Direction::S => {
                self.x.get(&x).and_then(|e| e.range(y..self.h).next().map(|y| (x, *y)))
            },
            Direction::E => {
                self.y.get(&y).and_then(|e| e.range(x..self.w).next().map(|x| (*x, y)))
            },
            Direction::W => {
                self.y.get(&y).and_then(|e| e.range(0..x).rev().next().map(|x| (*x, y)))
            },
        }
    }
}

struct Platform {
    round_rocks: BidiMap,
    cube_rocks: BidiMap,
}

impl FromStr for Platform {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut round_rocks = BidiMap::default();
        let mut cube_rocks = BidiMap::default();
        for (y, line) in s.split('\n').enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        cube_rocks.insert(x, y);
                    }
                    'O' => {
                        round_rocks.insert(x, y);
                    }
                    _ => {}
                }
                round_rocks.w = x + 1;
                cube_rocks.w = x + 1;
            }
            round_rocks.h = y + 1;
            cube_rocks.h = y + 1;
        }
        Ok(Self {
            round_rocks,
            cube_rocks,
        })
    }
}

impl Platform {
    fn tilt(&mut self, direction: Direction) {
        // we handle rocks in the order that they will be not blocked when moving in the tilt
        // direction.
        let ordered_rocks: Box<dyn Iterator<Item = (usize, usize)>> = match direction {
            Direction::N => Box::new(self.round_rocks.iter_by_y()),
            Direction::S => Box::new(self.round_rocks.iter_by_y().rev()),
            Direction::E => Box::new(self.round_rocks.iter_by_x().rev()),
            Direction::W => Box::new(self.round_rocks.iter_by_x()),
        };
        for (x, y) in ordered_rocks {
            let cube_blocker = self.cube_rocks.first_after(x, y, direction);
            let round_blocker = self.round_rocks.first_after(x, y, direction);
            let (dx, dy) = direction.opposite().offset();
            let (nx, ny) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
            self.round_rocks.remove(x, y);
            self.round_rocks.insert(nx, ny)
        }
    }
}
