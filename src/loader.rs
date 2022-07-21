use crate::{block::Block};
use ggez::{graphics::Image, Context, GameResult};

#[derive(Debug)]
pub struct Assets {
    pub air: Image,
    pub dirt: Image,
    pub stone: Image,
    pub ladder: Image,
    pub miner: Image,
    pub support: Image,
}

impl Assets {
    pub fn load(ctx: &mut Context) -> GameResult<Assets> {
        Ok(Assets {
            air: Image::from_path(ctx, "/air.png", true)?,
            dirt: Image::from_path(ctx, "/dirt.png", true)?,
            stone: Image::from_path(ctx, "/stone.png", true)?,
            ladder: Image::from_path(ctx, "/ladder.png", true)?,
            miner: Image::from_path(ctx, "/miner.png", true)?,
            support: Image::from_path(ctx, "/support.png", true)?,
        })
    }

    pub fn get(&self, block: &Block) -> &Image {
        match block {
            Block::Air => &self.air,
            Block::Dirt(_, _) => &self.dirt,
            Block::Stone(_) => &self.stone,
            Block::Ladder => &self.ladder,
        }
    }
}
