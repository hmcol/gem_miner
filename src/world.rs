use std::collections::{HashSet, VecDeque};

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
    pub gravity_list: Vec<Coord>,
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
            gravity_list: Vec::new(),
        }
    }

    pub fn miner_move(&mut self, dir: Direction) -> bool {
        // ensure target is a valid coordinate
        if let Some(c) = self.miner.offset_dir(dir) {
            // check if the coordinate contains a block which can be entered
            if matches!(self.block.get(c), Some(Block::Air | Block::Ladder)) {
                // special case for moving up
                if dir == NORTH && !matches!(self.block.get(self.miner), Some(Block::Ladder)) {
                    // if not in a ladder, try* to place a ladder first
                    self.place_ladder();
                }

                // at this point, we are permitted to move the miner

                self.miner = c;
                return true;
            }
        }

        false
    }

    pub fn miner_dig(&mut self, dir: Direction) -> bool {
        // ensure the target is a valid coordinate
        let c = match self.miner.offset_dir(dir) {
            Some(c) => c,
            None => return false,
        };

        // ensure the coordinate is on the map (i.e. contains a block)
        let block = match self.block.get_mut(c) {
            Some(b) => b,
            None => return false,
        };

        // check if the block is breakable (i.e. dirt)
        if let Block::Dirt(_, dmg) = block {
            // if the dirt has remaining damage, simply decrement and return
            if *dmg > 0 {
                *dmg -= 1;
                return false;
            }
        } else {
            return false;
        }

        // at this point we are going to break the block

        // set block to air
        *block = Block::Air;

        // extend any supports above or below into the new air block
        if let Some(true) = self.support.get_offset_dir(c, NORTH) {
            self.place_support(c);
        } else if let Some(true) = self.support.get_offset_dir(c, SOUTH) {
            self.place_support(c);
        }

        // enqueue block above to be checked for gravity
        if let Some(up) = c.offset_dir(NORTH) {
            self.gravity_list.push(up);
        }

        // done breaking the block
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

    pub fn gravity_check(&mut self) {
        let mut new_list = Vec::new();

        for & c in &self.gravity_list {
            if !matches!(self.block.get_offset_dir(c, SOUTH), Some(Block::Air)) {
                continue;
            }

            match self.block.get(c) {
                Some(Block::Ladder) => {
                    if let Some(down) = c.offset_dir(SOUTH) {
                        if self.block.set(down, Block::Ladder) {
                            self.block.set(c, Block::Air);
                            new_list.push(down);
    
                            if let Some(up) = c.offset_dir(NORTH) {
                                new_list.push(up);
                            }
                        }
                    }
                }
                Some(Block::Stone(0)) => {
                    if let Some(down) = c.offset_dir(SOUTH) {
                        if self.block.set(down, Block::new_stone()) {
                            self.block.set(c, Block::Air);
                            new_list.push(down);
    
                            if let Some(up) = c.offset_dir(NORTH) {
                                new_list.push(up);
                            }
                        }
                    }
                    
                }
                Some(&Block::Stone(timer)) => {
                    if let Some(false) = self.support.get_offset_dir(c, SOUTH) {
                        self.block.set(c, Block::Stone(timer - 1));
                        new_list.push(c);
                    }
                }
                _ => (),
            }
        }

        self.gravity_list = new_list;
    }

}
