use anyhow::{Context, Result};
use std::{
    cmp::{max, min},
    collections::{BTreeMap, HashMap},
    fs::read_to_string,
};

type Range = (u64, u64);
type RangeMap = BTreeMap<Range, Range>;
type MaterialMaps = HashMap<String, (String, RangeMap)>;
type Seeds = Vec<u64>;

const SEED: &str = "seed";
const LOCATION: &str = "location";

fn main() -> Result<()> {
    let contents =
        read_to_string("inputs/05.txt").context("Should have been able to read the file")?;
    let (seeds, mut material_maps) = parse(&contents)?;
    compress_material_maps(&mut material_maps)?;
    let min_location = seeds
        .iter()
        .map(|s| seed_to_location(*s, &material_maps))
        .min()
        .context("should have found a minimum")?;
    let min_range_location = seeds
        .chunks(2)
        .map(|s| seed_range_to_location(s, &material_maps))
        .min()
        .context("should have found a minimum")?;
    println!("part 1: {min_location}");
    println!("part 2: {min_range_location}");
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

fn parse_range(raw: &str) -> Result<(Range, Range)> {
    let mut nums = raw
        .split_whitespace()
        .map(|s| s.parse().context("failed to parse num"));
    let start_to = nums.next().context("should have start to")??;
    let start_from = nums.next().context("should have start from")??;
    let size = nums.next().context("should have size")??;
    let from_range = (start_from, start_from + size - 1);
    let to_range = (start_to, start_to + size - 1);
    Ok((from_range, to_range))
}

fn seed_to_location(seed: u64, material_maps: &MaterialMaps) -> u64 {
    let mut source_lookup = seed;
    let mut source = SEED;
    loop {
        let (destination, ranges) = &material_maps[source];
        let mut destination_lookup = source_lookup;
        for (from, to) in ranges {
            if range_contains(*from, source_lookup) {
                let offset = source_lookup - from.0;
                destination_lookup = to.0 + offset;
            }
        }
        source = destination;
        source_lookup = destination_lookup;
        if destination == LOCATION {
            break;
        }
    }
    source_lookup
}

fn range_contains((from, to): Range, val: u64) -> bool {
    from <= val && val <= to
}

fn compress_material_maps(material_maps: &mut MaterialMaps) -> Result<()> {
    // to compress the material map we take the "last" pair of conversions and
    // compress them into 1.
    // eg in an A -> B -> C -> D map, we will repeat until we have only A -> D
    // Take B -> C -> D and remove C by reducing the mappings from B to D directly.
    // In order to do so we need to split the ranges such that there are no
    // overlaps in the middle (B)
    fill_gaps(material_maps);
    loop {
        if material_maps.len() == 1 {
            break;
        }
        let (b, a_b_ranges) = material_maps.remove(SEED).context("missing seed")?;
        let (c, b_c_ranges) = material_maps.remove(&b).context("missing destination")?;
        // to split b we should reduce the first map into pairs and sort by the second item
        let mut a_b_ranges = a_b_ranges.into_iter().collect::<Vec<_>>();
        a_b_ranges.sort_by(|(_, a), (_, b)| a.cmp(b));
        let mut a_c_ranges = vec![];
        let mut i_a_b_ranges = a_b_ranges.iter();
        let mut i_b_c_ranges = b_c_ranges.iter();
        let mut a_b_range = i_a_b_ranges.next();
        let mut b_c_range = i_b_c_ranges.next();
        loop {
            match (a_b_range, b_c_range) {
                (None, None) => break,
                (Some(range), None) => {
                    a_c_ranges.push(*range);
                    a_b_range = i_a_b_ranges.next();
                }
                (None, Some((from, to))) => {
                    a_c_ranges.push((*from, *to));
                    b_c_range = i_b_c_ranges.next();
                }
                (
                    Some(((a_start, _), (a_b_start, a_b_end))),
                    Some(((b_c_start, b_c_end), (c_start, _))),
                ) => {
                    // insert only the overlaps here
                    let overlap_start = max(a_b_start, b_c_start);
                    let overlap_end = min(a_b_end, b_c_end);
                    let overlap_size = overlap_end - overlap_start;
                    let a_overlap_start = a_start + (overlap_start - a_b_start);
                    let a_overlap_end = a_overlap_start + overlap_size;
                    let c_overlap_start = c_start + (overlap_start - b_c_start);
                    let c_overlap_end = c_overlap_start + overlap_size;
                    a_c_ranges.push((
                        (a_overlap_start, a_overlap_end),
                        (c_overlap_start, c_overlap_end),
                    ));
                    if overlap_end == a_b_end {
                        a_b_range = i_a_b_ranges.next();
                    }
                    if overlap_end == b_c_end {
                        b_c_range = i_b_c_ranges.next();
                    }
                }
            }
        }
        material_maps.insert(SEED.to_string(), (c, a_c_ranges.into_iter().collect()));
    }
    Ok(())
}

fn fill_gaps(material_maps: &mut MaterialMaps) {
    for (_, ref mut ranges) in material_maps.values_mut() {
        let mut ranges_to_insert = vec![];
        let mut last_range = None;
        for (a_range, _) in ranges.iter() {
            match (last_range, a_range) {
                (None, (0, _)) => {}
                (None, (from, _)) => {
                    ranges_to_insert.push((0, *from));
                }
                (Some(&(_, last_to)), (from, _)) if last_to < from - 1 => {
                    ranges_to_insert.push((last_to + 1, *from - 1))
                }
                _ => {}
            }
            last_range = Some(a_range);
        }
        for range in ranges_to_insert {
            ranges.insert(range, range);
        }
    }
}

fn seed_range_to_location(seed_range: &[u64], material_maps: &MaterialMaps) -> u64 {
    let seed_start = seed_range[0];
    let seed_end = seed_start + seed_range[1];

    // get the edge locations
    let start_location = seed_to_location(seed_start, material_maps);
    let end_location = seed_to_location(seed_end, material_maps);

    // then all the minimums in between
    let (destination, ranges) = &material_maps[SEED];
    assert_eq!(destination, LOCATION);
    let in_between_min = ranges
        .range((seed_start, 0)..=(seed_end, u64::MAX))
        .map(|(_, (s, _))| *s)
        .min()
        .expect("should have a minimum");
    min(start_location, min(end_location, in_between_min))
}
