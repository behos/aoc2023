use std::{collections::BTreeMap, fs::read_to_string, str::FromStr};

use anyhow::{Context, Error, Result};

#[derive(Debug)]
struct Schematic {
    numbers: BTreeMap<(isize, isize), u32>,
    symbols: BTreeMap<(isize, isize), char>,
}

impl FromStr for Schematic {
    type Err = Error;

    fn from_str(raw: &str) -> Result<Self> {
        let mut numbers = BTreeMap::new();
        let mut symbols = BTreeMap::new();

        for (x, row) in raw.split("\n").enumerate() {
            for (y, c) in row.chars().enumerate() {
                match c {
                    '.' => {}
                    c if c.is_digit(10) => {
                        let n = c.to_digit(10).context("should be number")?;
                        numbers.insert((x as isize, y as isize), n);
                    }
                    c => {
                        symbols.insert((x as isize, y as isize), c);
                    }
                }
            }
        }
        Ok(Self { numbers, symbols })
    }
}

impl Schematic {
    fn sum(&self) -> u32 {
        let mut total = 0;
        let mut current_number = None;
        // going from left to right since our keys are ordered.
        for ((x, y), n) in &self.numbers {
            current_number = match current_number {
                None => Some(((*x, *y), *n, self.is_adjacent(*x, *y))),
                Some(((cx, cy), cn, is_adjacent)) if cx == *x && cy == *y - 1 => {
                    Some(((*x, *y), cn * 10 + n, is_adjacent || self.is_adjacent(*x, *y)))
                }
                Some((_, cn, is_adjacent)) => {
                    if is_adjacent {
                        total += cn;
                    }
                    Some(((*x, *y), *n, self.is_adjacent(*x, *y)))
                }
            }
        }
        if let Some((_, cn, true)) = current_number {
            total += cn
        }
        total
    }

    fn is_adjacent(&self, x: isize, y: isize) -> bool {
        [
            (x - 1, y - 1),
            (x - 1, y),
            (x - 1, y + 1),
            (x, y - 1),
            (x, y + 1),
            (x + 1, y - 1),
            (x + 1, y),
            (x + 1, y + 1),
        ]
        .iter()
        .any(|k| self.symbols.contains_key(k))
    }
}

fn main() -> Result<()> {
    let contents = read_to_string("inputs/03.txt").expect("Should have been able to read the file");
    let trimmed = contents.trim();
    let schematic = Schematic::from_str(trimmed)?;
    println!("part 1: {}", schematic.sum());
    // println!("part 2: {}", minimum_cubes(&games));
    Ok(())
}
