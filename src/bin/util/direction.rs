#[derive(PartialEq, Eq, Clone, Copy)]
pub(crate) enum Direction {
    N,
    S,
    E,
    W,
}

impl Direction {
    pub(crate) fn offset(&self) -> (i32, i32) {
        match self {
            Direction::N => (-1, 0),
            Direction::S => (1, 0),
            Direction::E => (0, 1),
            Direction::W => (0, -1),
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
}
