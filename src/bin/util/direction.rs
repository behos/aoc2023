#![allow(unused)]

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub(crate) enum Direction {
    N,
    S,
    E,
    W,
}

impl Direction {
    pub(crate) fn offset(&self) -> (i32, i32) {
        match self {
            Direction::N => (0, -1),
            Direction::S => (0, 1),
            Direction::E => (1, 0),
            Direction::W => (-1, 0),
        }
    }

    pub(crate) fn is_opposite(&self, other: Direction) -> bool {
        self.opposite() == other
    }

    pub(crate) fn left(&self) -> Self {
        match self {
            Direction::N => Direction::W,
            Direction::W => Direction::S,
            Direction::S => Direction::E,
            Direction::E => Direction::N,
        }
    }

    pub(crate) fn right(&self) -> Self {
        match self {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
        }
    }

    pub(crate) fn opposite(&self) -> Self {
        self.right().right()
    }

    pub(crate) fn min(&self, p1: (usize, usize), p2: (usize, usize)) -> (usize, usize) {
        match (self, p1, p2) {
            (Direction::N, (_, y1), (_, y2)) => {
                if y1 > y2 {
                    p1
                } else {
                    p2
                }
            }
            (Direction::S, (_, y1), (_, y2)) => {
                if y1 > y2 {
                    p2
                } else {
                    p1
                }
            }
            (Direction::E, (x1, _), (x2, _)) => {
                if x1 > x2 {
                    p2
                } else {
                    p1
                }
            }
            (Direction::W, (x1, _), (x2, _)) => {
                if x1 > x2 {
                    p1
                } else {
                    p2
                }
            }
        }
    }

    pub(crate) fn move_fowrard(
        self,
        (x, y): (usize, usize),
        (mx, my): (usize, usize),
    ) -> Option<(usize, usize)> {
        let (dx, dy) = self.offset();
        let (nx, ny) = (x as i32 + dx, y as i32 + dy);
        if nx < 0 || nx >= mx as i32 || ny < 0 || ny >= my as i32 {
            None
        } else {
            Some((nx as usize, ny as usize))
        }
    }
}
