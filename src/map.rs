use crate::{
    pos::{Coord, Direction},
    HEIGHT, WIDTH,
};

#[derive(Debug)]
pub struct Map<T> {
    data: [[T; WIDTH]; HEIGHT],
}

impl<T: Copy> Map<T> {
    pub fn new_with(val: T) -> Self {
        Map::new_from([[val; WIDTH]; HEIGHT])
    }
}

impl<T> Map<T> {
    pub fn new_from(data: [[T; WIDTH]; HEIGHT]) -> Self {
        Map { data }
    }

    pub fn get(&self, c: Coord) -> Option<&T> {
        self.data.get(c.y)?.get(c.x)
    }

    pub fn get_offset(&self, c: Coord, dir: Direction) -> Option<&T> {
        self.get(c.offset(dir)?)
    }

    pub fn get_mut(&mut self, c: Coord) -> Option<&mut T> {
        self.data.get_mut(c.y)?.get_mut(c.x)
    }

    pub fn get_mut_offset(&mut self, c: Coord, dir: Direction) -> Option<&mut T> {
        self.get_mut(c.offset(dir)?)
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
