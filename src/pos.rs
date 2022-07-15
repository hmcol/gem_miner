#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub const NORTH: Direction = Direction::North;
pub const EAST: Direction = Direction::East;
pub const SOUTH: Direction = Direction::South;
pub const WEST: Direction = Direction::West;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    pub fn new(x: usize, y: usize) -> Coord {
        Coord { x, y }
    }

    pub fn as_tuple(self) -> (usize, usize) {
        (self.x, self.y)
    }

    pub fn offset(self, dir: Direction) -> Option<Coord> {
        let (mut x, mut y) = self.as_tuple();

        match dir {
            Direction::North => y = y.checked_sub(1)?,
            Direction::East => x = x.checked_add(1)?,
            Direction::South => y = y.checked_add(1)?,
            Direction::West => x = x.checked_sub(1)?,
        }

        Some(Coord::new(x, y))
    }
}

pub fn coord(x: usize, y: usize) -> Coord {
    Coord::new(x, y)
}
