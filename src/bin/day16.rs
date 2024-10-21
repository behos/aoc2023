mod util;

use anyhow::{bail, Context, Error, Result};
use std::{fs::read_to_string, str::FromStr};
use util::bidimap::{BidiMap, Parsed};

enum Mirror {
    Forward,  // \
    Backward, // /
    Horizontal,
    Vertical,
}

impl TryFrom<char> for Parsed<Mirror> {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let item = match value {
            '\\' => Parsed::Item(Mirror::Forward),
            '/' => Parsed::Item(Mirror::Backward),
            '-' => Parsed::Item(Mirror::Horizontal),
            '|' => Parsed::Item(Mirror::Vertical),
            '.' => Parsed::Skip,
            _ => bail!("unknown char"),
        };
        Ok(item)
    }
}

type Grid = BidiMap<Mirror>;

fn main() -> Result<()> {
    let raw = read_to_string("inputs/16.txt").context("Should have been able to read the file")?;
    let grid = Grid::from_str(&raw)?;
    // println!("part 1: {focal_sum}");
    // println!("part 2: {focal_sum}");
    Ok(())
}
