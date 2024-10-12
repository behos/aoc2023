use std::{fs::read_to_string, str::FromStr, hash::{Hash, DefaultHasher, Hasher}};

use anyhow::{Context, Error, Result};
use cached::proc_macro::cached;

fn main() -> Result<()> {
    let raw = read_to_string("inputs/12.txt").context("Should have been able to read the file")?;
    let raw = raw.trim();
    let total = solve_for_input(raw)?;
    println!("part 1: {total}");
    let part_2_input = expand_input(raw);
    let total = solve_for_input(&part_2_input)?;
    println!("part 2: {total}");
    Ok(())
}

fn expand_input(raw: &str) -> String {
    raw.split('\n').map(expand_line).collect::<Vec<_>>().join("\n")
}

fn expand_line(raw: &str) -> String {
    let mut parts = raw.split(' ');
    let schema = parts.next().expect("should have schema");
    let schema = [schema; 5].join("?");
    let groups = parts.next().expect("should have groups");
    let groups = [groups; 5].join(",");
    format!("{schema} {groups}")
}

fn solve_for_input(raw: &str) -> Result<usize> {
    let mut total = 0;
    for line in raw.split('\n') {
        total += Entry::from_str(line)
            .context("failed to parse")?
            .arrangements();
    }
    Ok(total)
}

type Record = Vec<Condition>;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for Condition {
    fn from(value: char) -> Self {
        match value {
            '?' => Self::Unknown,
            '.' => Self::Operational,
            '#' => Self::Damaged,
            _ => panic!("unknown char"),
        }
    }
}

#[derive(Debug)]
struct Entry {
    record: Record,
    groups: Vec<usize>,
}

impl FromStr for Entry {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        let record = parts
            .next()
            .context("must have first part")?
            .chars()
            .map(Condition::from)
            .collect();
        let groups = parts
            .next()
            .context("must have next")?
            .split(',')
            .map(|s| s.parse::<usize>().context("failed to parse"))
            .collect::<Result<Vec<_>>>()
            .context("should be numbers")?;
        Ok(Self { record, groups })
    }
}

impl Entry {
    fn arrangements(&self) -> usize {
        try_arrangements(&self.record, &self.groups)
    }
}

#[cached(
    convert = "{ cache_hash(record, groups) }",
    key = "u64",
)]
fn try_arrangements(record: &[Condition], groups: &[usize]) -> usize {
    match (record, groups) {
        // If we don't have any more groups, then this arrangement works
        // if there are no remaining damaged conditions in the record.
        (r, []) => {
            if has_damaged(r) {
                0
            } else {
                1
            }
        }
        ([], [_, ..]) => 0,
        ([Condition::Operational, rest @ ..], g) => try_arrangements(rest, g),
        ([Condition::Damaged, rest @ ..], [g, grest @ ..]) => {
            if rest.len() < g - 1 || has_operational(&rest[..(g - 1)]) {
                0
            } else {
                let mut next_slice = &rest[(g - 1)..];
                if !next_slice.is_empty() {
                    if next_slice[0] == Condition::Damaged {
                        return 0;
                    } else {
                        next_slice = &next_slice[1..]
                    }
                }
                try_arrangements(next_slice, grest)
            }
        }
        ([Condition::Unknown, rest @ ..], g) => {
            let mut as_operational = vec![Condition::Operational];
            as_operational.extend(rest);
            let mut as_damaged = vec![Condition::Damaged];
            as_damaged.extend(rest);
            try_arrangements(&as_operational, g) + try_arrangements(&as_damaged, g)
        }
    }
}

fn cache_hash(record: &[Condition], groups: &[usize]) -> u64 {
    let mut s = DefaultHasher::new();
    record.hash(&mut s);
    groups.hash(&mut s);
    s.finish()
}

fn has_damaged(record: &[Condition]) -> bool {
    record.iter().any(|c| *c == Condition::Damaged)
}

fn has_operational(record: &[Condition]) -> bool {
    record.iter().any(|c| *c == Condition::Operational)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn single_rows() {
        let entry = Entry::from_str("???.### 1,1,3").unwrap();
        assert_eq!(1, entry.arrangements());
        let entry = Entry::from_str("?###???????? 3,2,1").unwrap();
        assert_eq!(10, entry.arrangements());
        let entry = Entry::from_str("????.######..#####. 1,6,5").unwrap();
        assert_eq!(4, entry.arrangements());
    }

    #[test]
    fn example() {
        let input = r#"
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
"#
        .trim();
        assert_eq!(21, solve_for_input(input).unwrap());
    }
}
