use crate::util::checked_add_signed;

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

    pub fn offset(self, ic: ICoord) -> Option<Coord> {
        let x = checked_add_signed(self.x, ic.x)?;
        let y = checked_add_signed(self.y, ic.y)?;

        Some(Coord::new(x, y))
    }

    pub fn offset_dir(self, dir: Direction) -> Option<Coord> {
        self.offset(dir.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ICoord {
    pub x: isize,
    pub y: isize,
}

impl ICoord {
    pub fn new(x: isize, y: isize) -> ICoord {
        ICoord { x, y }
    }
}

impl From<Direction> for ICoord {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::North => ICoord::new(0, -1),
            Direction::East => ICoord::new(1, 0),
            Direction::South => ICoord::new(0, 1),
            Direction::West => ICoord::new(-1, 0),
        }
    }
}

pub fn coord(x: usize, y: usize) -> Coord {
    Coord::new(x, y)
}

pub fn icoord(x: isize, y: isize) -> ICoord {
    ICoord::new(x, y)
}