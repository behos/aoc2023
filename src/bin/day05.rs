use anyhow::{Context, Result};
use std::{collections::HashMap, fs::read_to_string, ops::Range};

type RangeMap = HashMap<Range<u64>, Range<u64>>;
type MaterialMaps = HashMap<String, (String, RangeMap)>;
type Seeds = Vec<u64>;

fn main() -> Result<()> {
    let contents = read_to_string("inputs/05.txt").expect("Should have been able to read the file");
    let (seeds, material_maps) = parse(&contents)?;
    let min_location = seeds
        .iter()
        .map(|s| seed_to_location(*s, &material_maps))
        .min()
        .context("should have found a minimum")?;
    println!("part 1: {min_location}");
    // println!("part 2: {}", sum_of_calibrations_spelled_out(trimmed));
    Ok(())
}

fn parse(raw: &str) -> Result<(Seeds, MaterialMaps)> {
    let mut sections = raw.trim().split("\n\n");
    let seeds = parse_seeds(sections.next().context("should have seed section")?)?;
    let material_maps = sections
        .map(parse_material_map)
        .collect::<Result<MaterialMaps>>()?;
    Ok((seeds, material_maps))
}

fn parse_seeds(raw: &str) -> Result<Seeds> {
    raw.split(": ")
        .last()
        .expect("should have parts")
        .split_whitespace()
        .map(|s| s.parse().context("failed to parse number"))
        .collect()
}

fn parse_material_map(raw: &str) -> Result<(String, (String, RangeMap))> {
    let mut lines = raw.split('\n');
    let mut title = lines
        .next()
        .context("should have title")?
        .split_whitespace()
        .next()
        .context("should have first part")?
        .split("-to-");
    let (from, to) = (
        title.next().context("should have from")?.to_string(),
        title.next().context("should have to")?.to_string(),
    );

    let range_map = lines.map(parse_range).collect::<Result<_>>()?;
    Ok((from, (to, range_map)))
}

fn parse_range(raw: &str) -> Result<(Range<u64>, Range<u64>)> {
    let mut nums = raw
        .split_whitespace()
        .map(|s| s.parse().context("failed to parse num"));
    let start_to = nums.next().context("should have start to")??;
    let start_from = nums.next().context("should have start from")??;
    let size = nums.next().context("should have size")??;
    let from_range = start_from..(start_from + size);
    let to_range = start_to..(start_to + size);
    Ok((from_range, to_range))
}

fn seed_to_location(seed: u64, material_maps: &MaterialMaps) -> u64 {
    let mut source_lookup = seed;
    let mut source = "seed";
    loop {
        let (destination, ranges) = &material_maps[source];
        let mut destination_lookup = source_lookup;
        for (from, to) in ranges {
            if from.contains(&source_lookup) {
                let offset = source_lookup - from.start;
                destination_lookup = to.start + offset;
            }
        }
        source = destination;
        source_lookup = destination_lookup;
        if destination == "location" {
            break;
        }
    }
    source_lookup
}
