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

pub const ORES: [Ore; 14] = [
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
}
