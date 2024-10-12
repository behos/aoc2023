use std::{collections::BTreeSet, fs::read_to_string, str::FromStr};

use anyhow::{Context, Error, Result};

fn main() -> Result<()> {
    let raw = read_to_string("inputs/11.txt").context("Should have been able to read the file")?;
    let raw = raw.trim();
    let diagram = Diagram::from_str(raw)?;
    let distances = diagram.pair_distances(2);
    println!("part 1: {distances}");
    let distances = diagram.pair_distances(1_000_000);
    println!("part 2: {distances}");
    Ok(())
}

type Point = (usize, usize);

#[derive(Debug)]
struct Diagram {
    galaxies: BTreeSet<Point>,
    expanding_rows: BTreeSet<usize>,
    expanding_cols: BTreeSet<usize>,
}

impl FromStr for Diagram {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let galaxies: BTreeSet<_> = s
            .split('\n')
            .enumerate()
            .flat_map(|(x, line)| line.chars().enumerate().map(move |(y, c)| (x, y, c)))
            .filter_map(|(x, y, c)| if c == '#' { Some((x, y)) } else { None })
            .collect();
        let (galaxy_rows, galaxy_cols): (BTreeSet<_>, BTreeSet<_>) =
            galaxies.iter().cloned().unzip();
        let min_row = *galaxy_rows.first().context("has min")?;
        let max_row = *galaxy_rows.last().context("has min")?;
        let min_col = *galaxy_cols.first().context("has_min")?;
        let max_col = *galaxy_cols.last().context("has_max")?;
        let expanding_rows = (min_row..max_row)
            .filter(|x| !galaxy_rows.contains(x))
            .collect();
        let expanding_cols = (min_col..max_col)
            .filter(|y| !galaxy_cols.contains(y))
            .collect();
        Ok(Self {
            galaxies,
            expanding_rows,
            expanding_cols,
        })
    }
}

impl Diagram {
    fn pair_distances(&self, multiplier: usize) -> usize {
        let mut sum = 0;
        for (x1, y1) in &self.galaxies {
            for (x2, y2) in self.galaxies.range((*x1, *y1)..).skip(1) {
                let mut x = [*x1, *x2];
                x.sort();
                let mut y = [*y1, *y2];
                y.sort();
                let mut distance = x[1] - x[0] + y[1] - y[0];
                distance += self.expanding_cols.range(y[0]..y[1]).count() * (multiplier - 1);
                distance += self.expanding_rows.range(x[0]..x[1]).count() * (multiplier - 1);
                sum += distance;
            }
        }
        sum
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = r#"
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"#
        .trim();
        let diagram = Diagram::from_str(input).expect("should be parseable");
        println!("{diagram:?}");
        assert_eq!(374, diagram.pair_distances(2));
    }
}
