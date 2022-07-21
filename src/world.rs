use crate::{
    block::{Block, Ore},
    map::Map,
    pos::{coord, Coord, Direction, NORTH, SOUTH},
    HEIGHT, WIDTH,
};
use rand::Rng;

#[derive(Debug)]
pub struct World {
    pub block: Map<Block>,
    pub support: Map<bool>,
    pub miner: Coord,
}

impl World {
    pub fn new() -> World {
        let mut block = Map::new_with(Block::new_dirt());
        let mut rng = rand::thread_rng();

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let pos = Coord::new(x, y);
                if let Some(b) = block.get_mut(pos) {
                    if rng.gen_ratio(1, 10) {
                        *b = Block::new_stone();
                    }

                    if 2 < y && y < 17 && rng.gen_ratio(1, 10) {
                        *b = Block::Dirt(Some(Ore::Coal), 5);
                    }
                }
            }
        }

        for x in 0..WIDTH {
            block.set(coord(x, 0), Block::Air);
            block.set(coord(x, HEIGHT - 1), Block::new_stone());
        }

        for y in 1..HEIGHT {
            block.set(coord(0, y), Block::new_stone());
            block.set(coord(WIDTH - 1, y), Block::new_stone());
        }

        World {
            block,
            support: Map::new_with(false),
            miner: Coord::new(WIDTH / 2, 0),
        }
    }

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
