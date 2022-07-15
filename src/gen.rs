use rand::Rng;

use crate::{
    block::{Block, Ore},
    map::Map,
    pos::{coord, Coord},
    HEIGHT, WIDTH,
};
use std::convert::TryInto;

const ORES: [Ore; 14] = [
    Ore::Coal,
    Ore::Iron,
    Ore::WhiteOpal,
    Ore::GreenOpal,
    Ore::Silver,
    Ore::Gold,
    Ore::Ruby,
    Ore::RedOpal,
    Ore::Emerald,
    Ore::BlackOpal,
    Ore::Sapphire,
    Ore::Diamond,
    Ore::Uranium,
    Ore::Platinum,
];

pub fn new_map() -> Map<Block> {
    let mut map = Map::new_with(Block::new_dirt());

    let mut rng = rand::thread_rng();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let pos = Coord::new(x, y);
            if let Some(b) = map.get_mut(pos) {
                if rng.gen_ratio(1, 10) {
                    *b = Block::new_stone();
                }

                if 2 < y && y < 17 && rng.gen_ratio(1, 10) {
                    *b = Block::Dirt(Some(Ore::Coal), 5);
                }
            }
        }
    }

    // map[3][20] = Block::Dirt(Some(Ore::Coal), 3);
    // map[4][20] = Block::Dirt(Some(Ore::Iron), 3);
    // map[5][20] = Block::Dirt(Some(Ore::GreenOpal), 3);
    // map[6][20] = Block::Dirt(Some(Ore::WhiteOpal), 3);
    // map[7][20] = Block::Dirt(Some(Ore::Silver), 3);
    // map[8][20] = Block::Dirt(Some(Ore::Gold), 3);
    // map[9][20] = Block::Dirt(Some(Ore::Ruby), 3);
    // map[10][20] = Block::Dirt(Some(Ore::RedOpal), 3);
    // map[11][20] = Block::Dirt(Some(Ore::Emerald), 3);
    // map[12][20] = Block::Dirt(Some(Ore::BlackOpal), 3);
    // map[13][20] = Block::Dirt(Some(Ore::Sapphire), 3);
    // map[14][20] = Block::Dirt(Some(Ore::Diamond), 3);
    // map[15][20] = Block::Dirt(Some(Ore::Uranium), 3);
    // map[16][20] = Block::Dirt(Some(Ore::Platinum), 3);

    for x in 0..WIDTH {
        map.set(coord(x, 0), Block::Air);
        map.set(coord(x, HEIGHT - 1), Block::new_stone());
    }

    for y in 0..HEIGHT {
        map.set(coord(0, y), Block::new_stone());
        map.set(coord(WIDTH - 1, y), Block::new_stone());
    }

    map
}
