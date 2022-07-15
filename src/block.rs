use std::path::Path;

use image::io::Reader as ImageReader;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ore {
    Coal,
    Iron,
    GreenOpal,
    WhiteOpal,
    Silver,
    Gold,
    Ruby,
    RedOpal,
    Emerald,
    BlackOpal,
    Sapphire,
    Diamond,
    Uranium,
    Platinum,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Block {
    Air,
    Dirt(Option<Ore>, u8),
    Stone(u8),
    Ladder,
}

impl Block {
    pub fn new_dirt() -> Block {
        Block::Dirt(None, 3)
    }

    pub fn new_stone() -> Block {
        Block::Stone(8)
    }

    pub fn is_open(self) -> bool {
        matches!(self, Block::Air | Block::Ladder)
    }

    pub fn is_fall(self) -> bool {
        matches!(self, Block::Stone(_) | Block::Ladder)
    }

    pub fn tile(self) -> image::ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>> {
        let filename = match self {
            Block::Air =>  "air",
            Block::Dirt(_, _) => "dirt",
            Block::Stone(_) => "stone",
            Block::Ladder => "ladder",
        };

        ImageReader::open(format!("./assets/{}.png", filename))
            .unwrap()
            .decode()
            .unwrap()
            .into_rgba8()
    }
}
