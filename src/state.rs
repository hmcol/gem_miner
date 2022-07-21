use crate::{
    block::Block,
    loader::{Assets, Tile},
    pos::*,
    world::World,
    HEIGHT, TILE_SIZE, WIDTH,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Idle,
    Up,
    Right,
    Down,
    Left,
}
#[derive(Debug)]
pub struct State {
    world: World,
    command: Command,
}

impl State {
    pub fn new() -> State {
        State {
            world: World::new(),
            command: Command::Idle,
        }
    }

    pub fn set_command(&mut self, cmd: Command) {
        self.command = cmd;
    }

    pub fn update(&mut self) {
        if let Some(Block::Air) = self.world.block.get_offset(self.world.miner, SOUTH) {
            self.world.miner_fall();
            return;
        }

        match self.command {
            Command::Idle => (),
            Command::Up => self.try_dir(NORTH),
            Command::Right => self.try_dir(EAST),
            Command::Down => self.try_dir(SOUTH),
            Command::Left => self.try_dir(WEST),
        }
    }

    fn try_dir(&mut self, dir: Direction) {
        if self.world.miner_move(dir) {
            self.command = Command::Idle;
            return;
        }

        if self.world.miner_dig(dir) {
            // self.world.miner_move(dir);
            self.command = Command::Idle;
        }
    }

    pub fn draw(&self, screen: &mut [u8], assets: &Assets) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let pos = coord(x, y);

                if let Some(block) = self.world.block.get(pos) {
                    draw_tile(screen, assets.air, pos);
                    draw_tile(screen, assets.get(block), pos);
                }
            }
        }

        draw_tile(screen, assets.miner, self.world.miner)
    }
}

fn draw_tile(screen: &mut [u8], tile: Tile, pos: Coord) {
    for by in 0..TILE_SIZE {
        for bx in 0..TILE_SIZE {
            let x = bx + pos.x * TILE_SIZE;
            let y = by + pos.y * TILE_SIZE;

            let i = x + y * WIDTH * TILE_SIZE;

            if let Some(pix) = screen.chunks_exact_mut(4).nth(i) {
                let color = tile[bx + by * TILE_SIZE];
                if color[3] != 0 {
                    pix.copy_from_slice(&color);
                }
            }
        }
    }
}
