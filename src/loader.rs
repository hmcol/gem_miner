use std::collections::HashMap;

use crate::{block::Block, TILE_SIZE};
use image::io::Reader as ImageReader;

pub type Color = [u8; 4];

pub type Tile = [[Color; TILE_SIZE]; TILE_SIZE];

#[derive(Debug)]
pub struct Assets {
    pub air: Tile,
    pub dirt: Tile,
    pub stone: Tile,
    pub ladder: Tile,
    pub miner: Tile,
}

impl Default for Assets {
    fn default() -> Self {
        Assets {
            air: load_tile("air"),
            dirt: load_tile("dirt"),
            stone: load_tile("stone"),
            ladder: load_tile("ladder"),
            miner: load_tile("miner"),
        }
    }
}

impl Assets {
    pub fn get(&self, block: &Block) -> Tile {
        match block {
            Block::Air => self.air,
            Block::Dirt(_, _) => self.dirt,
            Block::Stone(_) => self.stone,
            Block::Ladder => self.ladder,
        }
    }
}

fn load_tile(filename: &str) -> Tile {
    let img = ImageReader::open(format!("./assets/{}.png", filename))
        .unwrap()
        .decode()
        .unwrap()
        .into_rgba8();

    let mut tile = Tile::default();

    for y in 0..TILE_SIZE {
        for x in 0..TILE_SIZE {
            tile[y][x] = img.get_pixel(x as u32, y as u32).0;
        }
    }

    tile
}
