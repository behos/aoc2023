use std::{collections::HashSet, fs::read_to_string, str::FromStr};

use anyhow::{bail, Context, Error, Result};

fn main() -> Result<()> {
    let raw = read_to_string("inputs/11.txt").context("Should have been able to read the file")?;
    let raw = raw.trim();
    let diagram = Diagram::from_str(raw)?;
    // println!("part 1: {dist}");
    // println!("part 2: {counts:?}");
    Ok(())
}

type Point = (usize, usize);

struct Diagram {
    galaxies: HashSet<Point>,
    expanding_rows: HashSet<usize>,
    expanding_columns: HashSet<usize>,
}

impl FromStr for Diagram {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let galaxies = s
            .split('\n')
            .enumerate()
            .flat_map(|(x, line)| line.chars().enumerate().map(move |(y, c)| (x, y, c)))
            .filter_map(|(x, y, c)| if c == '#' { Some((x, y)) } else { None })
            .collect();
        Ok(Self {
            galaxies,
            expanding_rows: todo!(),
            expanding_columns: todo!(),
        })
    }
}
