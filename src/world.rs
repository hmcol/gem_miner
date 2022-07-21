use std::collections::VecDeque;

use crate::{
    block::{Block, Ore},
    map::Map,
    pos::{coord, Coord, Direction, NORTH, SOUTH},
    WORLD_HEIGHT, WORLD_WIDTH,
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

        for y in 0..WORLD_HEIGHT {
            for x in 0..WORLD_WIDTH {
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

        for x in 0..WORLD_WIDTH {
            block.set(coord(x, 0), Block::Air);
            block.set(coord(x, WORLD_HEIGHT - 1), Block::new_stone());
        }

        for y in 1..WORLD_HEIGHT {
            block.set(coord(0, y), Block::new_stone());
            block.set(coord(WORLD_WIDTH - 1, y), Block::new_stone());
        }

        World {
            block,
            support: Map::new_with(false),
            miner: Coord::new(WORLD_WIDTH / 2, 0),
        }
    }

    pub fn miner_move(&mut self, dir: Direction) -> bool {
        if let Some(new_pos) = self.miner.offset_dir(dir) {
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
        let pos = match self.miner.offset_dir(dir) {
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

        if let Some(true) = self.support.get_offset_dir(pos, NORTH) {
            self.place_support(pos);
        } else if let Some(true) = self.support.get_offset_dir(pos, SOUTH) {
            self.place_support(pos);
        }

        true
    }

    pub fn miner_fall(&mut self) -> bool {
        match self.block.get_offset_dir(self.miner, SOUTH) {
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

    pub fn place_support(&mut self, coord: Coord) {
        let mut queue = VecDeque::new();

        queue.push_back(coord);

        while let Some(c) = queue.pop_front() {
            if let Some(Block::Air | Block::Ladder) = self.block.get(c) {
                // place a support at `c`
                self.support.set(c, true);

                // check if up has support
                if let Some(up) = c.offset_dir(NORTH) {
                    if let Some(false) = self.support.get(up) {
                        // if not, add to queue
                        queue.push_back(up);
                    }
                }

                // check if down has support
                if let Some(down) = c.offset_dir(SOUTH) {
                    if let Some(false) = self.support.get(down) {
                        // if not, add to queue
                        queue.push_back(down);
                    }
                }
            }
        }
    }
}
