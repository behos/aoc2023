mod util;

use std::{fs::read_to_string, str::FromStr};

use anyhow::{bail, Context, Error, Result};
use crate::util::direction::Direction;

fn main() -> Result<()> {
    let raw = read_to_string("inputs/10.txt").context("Should have been able to read the file")?;
    let raw = raw.trim();
    let diagram = Diagram::from_str(raw)?;

    let (loop_len, mut marked) = diagram.find_loop();
    let dist = loop_len / 2;
    println!("part 1: {dist}");
    fill_gaps(&mut marked);
    // print_markers(&marked);
    let counts = count_markers(&marked);
    println!("part 2: {counts:?}");
    Ok(())
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Point {
    Pipe(Direction, Direction),
    Ground,
    Start,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Marker {
    Left,
    Right,
    Loop,
}

impl TryFrom<char> for Point {
    type Error = Error;

    fn try_from(value: char) -> Result<Self> {
        Ok(match value {
            '|' => Self::Pipe(Direction::N, Direction::S),
            '-' => Self::Pipe(Direction::E, Direction::W),
            'L' => Self::Pipe(Direction::N, Direction::E),
            'J' => Self::Pipe(Direction::N, Direction::W),
            '7' => Self::Pipe(Direction::S, Direction::W),
            'F' => Self::Pipe(Direction::S, Direction::E),
            '.' => Self::Ground,
            'S' => Self::Start,
            c => bail!("unrecognised character {c}"),
        })
    }
}

struct Diagram {
    points: Vec<Vec<Point>>,
    start: (usize, usize),
    size: (usize, usize),
}

impl FromStr for Diagram {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut points = vec![];
        let mut start = (0, 0);
        for line in s.split('\n') {
            let mut line_points = vec![];
            for c in line.chars() {
                let point = Point::try_from(c)?;
                if point == Point::Start {
                    start = (line_points.len(), points.len());
                }
                line_points.push(point);
            }
            points.push(line_points);
        }
        let size = (points[0].len(), points.len());
        Ok(Self {
            points,
            start,
            size,
        })
    }
}

impl Diagram {
    fn step(
        &self,
        (cur_x, cur_y): (usize, usize),
        direction: Direction,
    ) -> Option<((usize, usize), Direction, Point)> {
        let next = point_offset(self.size, (cur_x, cur_y), direction)?;
        let next_point = &self.points[next.1][next.0];
        match next_point {
            Point::Start => Some((next, direction, *next_point)),
            Point::Pipe(d1, d2) if d1.is_opposite(direction) => Some((next, *d2, *next_point)),
            Point::Pipe(d1, d2) if d2.is_opposite(direction) => Some((next, *d1, *next_point)),
            _ => None,
        }
    }

    fn find_loop(&self) -> (usize, Vec<Vec<Option<Marker>>>) {
        for direction in [Direction::N, Direction::S, Direction::E, Direction::W] {
            let mut loop_path = vec![];
            let mut markers = vec![vec![None; self.points[0].len()]; self.points.len()];
            let mut current_pos = self.start;
            let mut current_dir = direction;
            while let Some((pos, dir, point)) = self.step(current_pos, current_dir) {
                current_pos = pos;
                mark_points(&mut markers, self.size, current_pos, &[current_dir, dir]);
                current_dir = dir;
                if current_pos == self.start {
                    let point = Point::Pipe(direction, current_dir);
                    loop_path.push((current_pos, point));
                    return (loop_path.len(), markers);
                }
                loop_path.push((current_pos, point));
            }
        }
        panic!("oh noes");
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum LoopPoint {
    Horizontal,
    Vertical,
}

impl From<Point> for LoopPoint {
    fn from(value: Point) -> Self {
        match value {
            Point::Pipe(Direction::N | Direction::S, _)
            | Point::Pipe(_, Direction::N | Direction::S) => Self::Vertical,
            Point::Pipe(_, _) => Self::Horizontal,
            Point::Ground => panic!("ground found in loop"),
            Point::Start => todo!("start found in loop"),
        }
    }
}

fn fill_gaps(markers: &mut Vec<Vec<Option<Marker>>>) {
    for x in 0..markers.len() {
        for y in 0..markers[x].len() {
            match markers[x][y] {
                Some(m @ Marker::Left) | Some(m @ Marker::Right) => {
                    mark_neighbors(markers, (x, y), m);
                }
                _ => {}
            }
        }
    }
}

fn mark_neighbors(markers: &mut Vec<Vec<Option<Marker>>>, (x, y): (usize, usize), marker: Marker) {
    let size = (markers.len(), markers[0].len());
    for direction in [Direction::N, Direction::S, Direction::E, Direction::W] {
        if let Some((nx, ny)) = point_offset(size, (x, y), direction) {
            if markers[nx][ny].is_none() {
                markers[nx][ny] = Some(marker);
                mark_neighbors(markers, (nx, ny), marker)
            }
        }
    }
}

fn in_bounds(size: (usize, usize), point: (i32, i32)) -> bool {
    point.0 >= 0 && point.1 >= 0 && point.0 < size.0 as i32 && point.1 < size.1 as i32
}

fn point_offset(
    size: (usize, usize),
    (x, y): (usize, usize),
    direction: Direction,
) -> Option<(usize, usize)> {
    let (offset_x, offset_y) = direction.offset();
    let next = (x as i32 + offset_x, y as i32 + offset_y);
    if !in_bounds(size, next) {
        return None;
    }
    Some((next.0 as usize, next.1 as usize))
}

fn count_markers(markers: &[Vec<Option<Marker>>]) -> (usize, usize) {
    markers
        .iter()
        .flatten()
        .fold((0, 0), |(l, r), marker| match marker {
            Some(Marker::Left) => (l + 1, r),
            Some(Marker::Right) => (l, r + 1),
            _ => (l, r),
        })
}

#[allow(unused)]
fn print_markers(markers: &Vec<Vec<Option<Marker>>>) {
    for row in markers {
        for marker in row {
            let c = match marker {
                Some(Marker::Left) => '*',
                Some(Marker::Right) => '_',
                Some(Marker::Loop) => '.',
                None => panic!("unmarked point"),
            };
            print!("{c}");
        }
        println!()
    }
}

fn mark_points(
    markers: &mut [Vec<Option<Marker>>],
    size: (usize, usize),
    current_pos: (usize, usize),
    directions: &[Direction],
) {
    markers[current_pos.0][current_pos.1] = Some(Marker::Loop);
    for direction in directions {
        for (direction, marker) in [
            (direction.left(), Marker::Left),
            (direction.right(), Marker::Right),
        ] {
            if let Some((x, y)) = point_offset(size, current_pos, direction) {
                let m = &mut markers[x][y];
                if m.is_none() {
                    m.replace(marker);
                }
            }
        }
    }
}
