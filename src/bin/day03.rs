use std::{
    collections::{BTreeMap, HashSet},
    fs::read_to_string,
    str::FromStr,
};

use anyhow::{Context, Error, Result};

#[derive(Debug)]
struct Schematic {
    numbers: BTreeMap<(isize, isize), u64>,
    symbols: BTreeMap<(isize, isize), char>,
}

type Connections = BTreeMap<(isize, isize), HashSet<u64>>;

impl FromStr for Schematic {
    type Err = Error;

    fn from_str(raw: &str) -> Result<Self> {
        let mut numbers = BTreeMap::new();
        let mut symbols = BTreeMap::new();

        for (x, row) in raw.split('\n').enumerate() {
            for (y, c) in row.chars().enumerate() {
                match c {
                    '.' => {}
                    c if c.is_ascii_digit() => {
                        let n = c.to_digit(10).context("should be number")?;
                        numbers.insert((x as isize, y as isize), n as u64);
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
    fn connections_and_sum(&self) -> (Connections, u64) {
        let mut total = 0;
        let mut current_number = None;
        let mut connections = Connections::new();
        // going from left to right since our keys are ordered.
        for ((x, y), n) in &self.numbers {
            let adjacent_symbols = self.adjacent_symbols(*x, *y);
            current_number = match current_number {
                None => Some(((*x, *y), *n, adjacent_symbols)),
                Some(((cx, cy), cn, mut cas)) if cx == *x && cy == *y - 1 => {
                    cas.extend(adjacent_symbols);
                    Some(((*x, *y), cn * 10 + n, cas))
                }
                Some((_, cn, cas)) => {
                    if !cas.is_empty() {
                        total += cn;
                    }
                    for (sx, sy) in cas {
                        connections.entry((sx, sy)).or_default().insert(cn);
                    }
                    Some(((*x, *y), *n, adjacent_symbols))
                }
            }
        }
        if let Some((_, cn, cas)) = current_number {
            if !cas.is_empty() {
                total += cn;
            }
            for (sx, sy) in cas {
                connections.entry((sx, sy)).or_default().insert(cn);
            }
        }
        (connections, total)
    }

    fn adjacent_symbols(&self, x: isize, y: isize) -> Vec<(isize, isize)> {
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
        .into_iter()
        .filter(|k| self.symbols.contains_key(k))
        .collect()
    }

    fn gear_ratios(&self, connections: &Connections) -> u64 {
        connections
            .iter()
            .filter_map(|((x, y), nums)| {
                if nums.len() == 2 && self.symbols.get(&(*x, *y)) == Some(&'*') {
                    Some(nums.iter().product::<u64>())
                } else {
                    None
                }
            })
            .sum()
    }
}

fn main() -> Result<()> {
    let contents = read_to_string("inputs/03.txt").expect("Should have been able to read the file");
    let trimmed = contents.trim();
    let schematic = Schematic::from_str(trimmed)?;
    let (connections, sum) = schematic.connections_and_sum();
    println!("part 1: {}", sum);
    println!("part 2: {}", schematic.gear_ratios(&connections));
    Ok(())
}
