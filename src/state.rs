use crate::{
    block::Block, loader::Assets, pos::*, world::World, VIEW_DIST_X, VIEW_DIST_Y, VIEW_HEIGHT,
    VIEW_WIDTH,
};
use ggez::graphics::{Canvas, DrawParam};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Idle,
    Up,
    Right,
    Down,
    Left,
    PlaceSupport,
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
        if let Some(Block::Air) = self.world.block.get_offset_dir(self.world.miner, SOUTH) {
            self.world.miner_fall();
            return;
        }

        match self.command {
            Command::Idle => (),
            Command::Up => self.try_dir(NORTH),
            Command::Right => self.try_dir(EAST),
            Command::Down => self.try_dir(SOUTH),
            Command::Left => self.try_dir(WEST),
            Command::PlaceSupport => {
                self.world.place_support(self.world.miner);
                self.command = Command::Idle;
            },
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

    pub fn draw(&self, canvas: &mut Canvas, assets: &Assets) {
        let tile_scale = match canvas.screen_coordinates() {
            Some(rect) => std::cmp::min(
                rect.w as usize / (VIEW_WIDTH * 8),
                rect.h as usize / (VIEW_HEIGHT * 8),
            ),
            None => return,
        } as f32;

        let ivdy = VIEW_DIST_Y as isize;
        let ivdx = VIEW_DIST_X as isize;

        for iy in -ivdy..=ivdy {
            for ix in -ivdx..=ivdx {
                let ic = icoord(ix, iy);

                let world_coord = match self.world.miner.offset(ic) {
                    Some(c) => c,
                    None => continue,
                };

                if let Some(true) = self.world.support.get(world_coord) {
                    canvas.draw(
                        &assets.support,
                        tile_draw_param([(ix + ivdx) as f32, (iy + ivdy) as f32], tile_scale),
                    );
                }

                if let Some(block) = self.world.block.get(world_coord) {
                    canvas.draw(
                        assets.get(block),
                        tile_draw_param([(ix + ivdx) as f32, (iy + ivdy) as f32], tile_scale),
                    );
                }
            }
        }

        canvas.draw(
            &assets.miner,
            tile_draw_param([VIEW_DIST_X as f32, VIEW_DIST_Y as f32], tile_scale),
        );
    }

    
}

fn tile_draw_param(p: [f32; 2], scale: f32) -> DrawParam {
    DrawParam::default()
        .scale([scale, scale])
        .dest([p[0] * scale * 8.0, p[1] * scale * 8.0])
}
