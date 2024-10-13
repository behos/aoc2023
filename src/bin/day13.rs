use std::{cmp::min, collections::BTreeSet, fs::read_to_string, str::FromStr};

use anyhow::{Context, Error, Result};

fn main() -> Result<()> {
    let raw = read_to_string("inputs/13.txt").context("Should have been able to read the file")?;
    let raw = raw.trim();
    let mut patterns = raw
        .split("\n\n")
        .map(|r| Pattern::from_str(r).context("unparseable pattern"))
        .collect::<Result<Vec<Pattern>>>()?;
    let pt1 = patterns.iter().map(Pattern::mirror).sum::<usize>();
    println!("part 1: {pt1}");

    let pt2 = patterns
        .iter_mut()
        .map(Pattern::smudged_mirror)
        .sum::<usize>();
    println!("part 2: {pt2}");
    Ok(())
}

struct Pattern {
    by_row: Vec<Vec<char>>,
    by_column: Vec<Vec<char>>,
}

impl FromStr for Pattern {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let by_row = s
            .split('\n')
            .map(|s| s.chars().collect())
            .collect::<Vec<Vec<_>>>();
        let mut by_column = vec![vec![' '; by_row.len()]; by_row[0].len()];
        for (x, row) in by_row.iter().enumerate() {
            for (y, c) in row.iter().enumerate() {
                by_column[y][x] = *c;
            }
        }
        Ok(Self { by_row, by_column })
    }
}

impl Pattern {
    fn mirror(&self) -> usize {
        let row_mirror = find_mirror(&self.by_row, None);
        let column_mirror = find_mirror(&self.by_column, None);
        row_mirror + 100 * column_mirror
    }

    fn smudged_mirror(&mut self) -> usize {
        let mirror = self.mirror();
        let exclude_x = Some(mirror % 100);
        let exclude_y = Some(mirror / 100);
        for x in 0..self.by_row.len() {
            for y in 0..self.by_column.len() {
                let old = self.by_row[x][y];
                let new = if old == '.' { '#' } else { '.' };
                self.by_row[x][y] = new;
                let row_mirror = find_mirror(&self.by_row, exclude_x);
                self.by_row[x][y] = old;
                if row_mirror > 0 {
                    return row_mirror;
                }
                self.by_column[y][x] = new;
                let column_mirror = find_mirror(&self.by_column, exclude_y);
                self.by_column[y][x] = old;
                if column_mirror > 0 {
                    return column_mirror * 100;
                }
            }
        }
        panic!("no solution")
    }
}

fn find_mirror(pattern: &[Vec<char>], exclude: Option<usize>) -> usize {
    let mut candidates = pattern
        .iter()
        .map(|p| mirror_points(p))
        .reduce(|acc, e| acc.intersection(&e).cloned().collect())
        .expect("must have a set");
    if let Some(exclude) = exclude {
        candidates.remove(&exclude);
    }
    candidates.pop_first().unwrap_or(0)
}

fn mirror_points(line: &[char]) -> BTreeSet<usize> {
    (1..line.len())
        .filter(|i| mirrors_at_point(*i, line))
        .collect()
}

fn mirrors_at_point(idx: usize, line: &[char]) -> bool {
    let length = min(idx, line.len() - idx);
    let start = idx - length;
    let end = idx + length;
    is_palindrome(&line[start..end])
}

fn is_palindrome(slice: &[char]) -> bool {
    slice.iter().zip(slice.iter().rev()).all(|(a, b)| a == b)
}
