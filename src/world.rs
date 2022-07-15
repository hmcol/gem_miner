use crate::{
    block::Block,
    gen,
    map::Map,
    pos::{Coord, Direction, NORTH, SOUTH},
    WIDTH,
};

#[derive(Debug)]
pub struct World {
    pub block: Map<Block>,
    pub support: Map<bool>,
    pub miner: Coord,
}

impl Default for World {
    fn default() -> Self {
        World {
            block: gen::new_map(),
            support: Map::new_with(false),
            miner: Coord::new(WIDTH / 2, 0),
        }
    }
}

impl World {
    pub fn miner_move(&mut self, dir: Direction) -> bool {
        if let Some(new_pos) = self.miner.offset(dir) {
            if matches!(self.block.get(new_pos), Some(Block::Air | Block::Ladder)) {
                if dir == NORTH && !matches!(self.block.get(self.miner), Some(Block::Ladder)) {
                    self.place_ladder();
                }

                self.miner = new_pos;
                return true;
            }
        }

        false
    }

    pub fn miner_dig(&mut self, dir: Direction) -> bool {
        let pos = match self.miner.offset(dir) {
            Some(c) => c,
            None => return false,
        };

        let block = match self.block.get_mut(pos) {
            Some(b) => b,
            None => return false,
        };

        if let Block::Dirt(_, dmg) = block {
            if *dmg > 0 {
                *dmg -= 1;
                return false;
            }
        } else {
            return false;
        }

        *block = Block::Air;

        if *self
            .support
            .get_offset(pos, NORTH)
            .or_else(|| self.support.get_offset(pos, SOUTH))
            .unwrap_or(&false)
        {
            self.place_support(pos);
        }

        true
    }

    pub fn miner_fall(&mut self) -> bool {
        match self.block.get_offset(self.miner, SOUTH) {
            Some(Block::Air) => {
                self.miner.y += 1;
                true
            }
            _ => false,
        }
    }

    pub fn place_ladder(&mut self) -> bool {
        self.block.set(self.miner, Block::Ladder)
    }

    fn place_support(&mut self, coord: Coord) -> Option<()> {
        let mut c = coord;
        while let Some(Block::Air) = self.block.get(c) {
            c = c.offset(NORTH)?;

            if self.support.get(c).copied().unwrap_or(true) {
                break;
            }

            self.support.set(c, true);
        }

        c = coord;
        while let Some(Block::Air) = self.block.get(c) {
            c = c.offset(SOUTH)?;

            if self.support.get(c).copied().unwrap_or(true) {
                break;
            }

            self.support.set(c, true);
        }

        Some(())
    }
}
