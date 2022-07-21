use crate::{
    pos::{Coord, Direction, ICoord},
    WORLD_HEIGHT, WORLD_WIDTH,
};

#[derive(Debug)]
pub struct Map<T> {
    data: [[T; WORLD_WIDTH]; WORLD_HEIGHT],
}

impl<T: Copy> Map<T> {
    pub fn new_with(val: T) -> Self {
        Map::new_from([[val; WORLD_WIDTH]; WORLD_HEIGHT])
    }
}

impl<T> Map<T> {
    pub fn new_from(data: [[T; WORLD_WIDTH]; WORLD_HEIGHT]) -> Self {
        Map { data }
    }

    pub fn get(&self, c: Coord) -> Option<&T> {
        self.data.get(c.y)?.get(c.x)
    }

    pub fn get_offset(&self, c: Coord, ic: ICoord) -> Option<&T> {
        self.get(c.offset(ic)?)
    }

    pub fn get_offset_dir(&self, c: Coord, dir: Direction) -> Option<&T> {
        self.get_offset(c, dir.into())
    }

    pub fn get_mut(&mut self, c: Coord) -> Option<&mut T> {
        self.data.get_mut(c.y)?.get_mut(c.x)
    }

    pub fn get_mut_offset(&mut self, c: Coord, ic: ICoord) -> Option<&mut T> {
        self.get_mut(c.offset(ic)?)
    }

    pub fn set(&mut self, c: Coord, val: T) -> bool {
        match self.get_mut(c) {
            Some(e) => {
                *e = val;
                true
            }
            None => false,
        }
    }
}
