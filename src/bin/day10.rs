use std::{fs::read_to_string, str::FromStr};

use anyhow::{bail, Context, Error, Result};

fn main() -> Result<()> {
    let raw = read_to_string("inputs/10.txt").context("Should have been able to read the file")?;
    let raw = raw.trim();
    let diagram = Diagram::from_str(raw)?;

    let loop_size = diagram.find_loop();
    let dist = loop_size / 2;
    println!("part 1: {dist}");
    // println!("part 2: {part_2}");
    Ok(())
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Direction {
    N,
    S,
    E,
    W,
}

impl Direction {
    fn offset(&self) -> (i32, i32) {
        match self {
            Direction::N => (-1, 0),
            Direction::S => (1, 0),
            Direction::E => (0, 1),
            Direction::W => (0, -1),
        }
    }

    fn is_opposite(&self, other: Direction) -> bool {
        matches!(
            (self, other),
            (Direction::N, Direction::S)
                | (Direction::S, Direction::N)
                | (Direction::E, Direction::W)
                | (Direction::W, Direction::E)
        )
    }
}

#[derive(PartialEq, Eq)]
enum Point {
    Pipe(Direction, Direction),
    Ground,
    Start,
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
                    start = (points.len(), line_points.len());
                }
                line_points.push(point);
            }
            points.push(line_points);
        }
        let size = (points.len(), points[0].len());
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
    ) -> Option<((usize, usize), Direction)> {
        let (offset_x, offset_y) = direction.offset();
        let next = (cur_x as i32 + offset_x, cur_y as i32 + offset_y);
        if next < (0, 0) {
            return None
        }
        let next = (next.0 as usize, next.1 as usize);
        if next >= self.size {
            return None
        }
        let next_point = &self.points[next.0][next.1];
        match next_point {
            Point::Start => Some((next, direction)),
            Point::Pipe(d1, d2) if d1.is_opposite(direction) => Some((next, *d2)),
            Point::Pipe(d1, d2) if d2.is_opposite(direction) => Some((next, *d1)),
            _ => None,
        }
    }

    fn find_loop(&self) -> usize {
        for direction in [Direction::N, Direction::S, Direction::E, Direction::W] {
            let mut current_pos = self.start;
            let mut current_dir = direction;
            let mut steps = 0;
            while let Some((pos, dir)) = self.step(current_pos, current_dir) {
                steps += 1;
                current_pos = pos;
                current_dir = dir;
                if current_pos == self.start {
                    return steps
                }
            }
        }
        panic!("oh noes");
    }
}
