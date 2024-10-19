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
    let mut platform = Platform::from_str(raw).context("failed to parse platform")?;
    platform.tilt(Direction::N);
    let total_load = platform.total_load();
    println!("part 1: {total_load}");
    // println!("part 2: {pt2}");
    Ok(())
}

#[derive(Default)]
struct BidiMap {
    x: BTreeMap<usize, BTreeSet<usize>>,
    y: BTreeMap<usize, BTreeSet<usize>>,
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

    fn iter_by_x(&self) -> impl DoubleEndedIterator<Item = (usize, usize)> + '_ {
        self.x
            .iter()
            .flat_map(|(x, ys)| ys.iter().map(|y| (*x, *y)))
    }

    fn iter_by_y(&self) -> impl DoubleEndedIterator<Item = (usize, usize)> + '_ {
        self.y
            .iter()
            .flat_map(|(y, xs)| xs.iter().map(|x| (*x, *y)))
    }

    fn first_after(&self, (x, y): (usize, usize), direction: Direction) -> Option<(usize, usize)> {
        match direction {
            Direction::N => self
                .x
                .get(&x)
                .and_then(|e| e.range(..y).next_back().map(|y| (x, *y))),
            Direction::S => self
                .x
                .get(&x)
                .and_then(|e| e.range(y..).next().map(|y| (x, *y))),
            Direction::E => self
                .y
                .get(&y)
                .and_then(|e| e.range(x..).next().map(|x| (*x, y))),
            Direction::W => self
                .y
                .get(&y)
                .and_then(|e| e.range(..x).next_back().map(|x| (*x, y))),
        }
    }
}

struct Platform {
    round_rocks: BidiMap,
    cube_rocks: BidiMap,
    w: usize,
    h: usize,
}

impl FromStr for Platform {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut round_rocks = BidiMap::default();
        let mut cube_rocks = BidiMap::default();
        let mut w = 0;
        let mut h = 0;
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
                w = x + 1;
            }
            h = y + 1;
        }
        Ok(Self {
            round_rocks,
            cube_rocks,
            w,
            h,
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
        let ordered_rocks: Vec<_> = ordered_rocks.collect();
        for (x, y) in ordered_rocks {
            let cube_blocker = self.cube_rocks.first_after((x, y), direction);
            let round_blocker = self.round_rocks.first_after((x, y), direction);
            let blocker = match (cube_blocker, round_blocker) {
                (Some(c), Some(r)) => Some(direction.min(c, r)),
                (None, None) => None,
                (None, b) | (b, None) => b,
            };
            let (nx, ny) = if let Some((x, y)) = blocker {
                let (dx, dy) = direction.opposite().offset();
                ((x as i32 + dx) as usize, (y as i32 + dy) as usize)
            } else {
                self.edge(direction, (x, y))
            };
            self.round_rocks.remove(x, y);
            self.round_rocks.insert(nx, ny)
        }
    }

    fn edge(&self, direction: Direction, (x, y): (usize, usize)) -> (usize, usize) {
        match direction {
            Direction::N => (x, 0),
            Direction::S => (x, self.h - 1),
            Direction::E => (self.w - 1, y),
            Direction::W => (0, y),
        }
    }

    fn total_load(&self) -> usize {
        self.round_rocks.iter_by_x().map(|(_, y)| self.h - y).sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let example = r#"
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
"#
        .trim();
        let mut platform = Platform::from_str(example).expect("should be parsed");
        platform.tilt(Direction::N);
        let rocks = platform.round_rocks.iter_by_y().collect::<BTreeSet<_>>();
        let cubes = platform.cube_rocks.iter_by_y().collect::<BTreeSet<_>>();
        for y in 0..10 {
            for x in 0..10 {
                if rocks.contains(&(x, y)) {
                    print!("O");
                } else if cubes.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        assert_eq!(136, platform.total_load());
    }
}
