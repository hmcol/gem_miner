mod block;
mod loader;
mod map;
mod pos;
mod state;
mod util;
mod world;

use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Canvas, CanvasLoadOp, Color, DrawParam, Image, Sampler};
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::{Context, ContextBuilder, GameResult};
use loader::Assets;
use state::{Command, State};

pub const WORLD_WIDTH: usize = 64;
pub const WORLD_HEIGHT: usize = 20;

pub const VIEW_DIST_X: usize = 4;
pub const VIEW_DIST_Y: usize = 3;

pub const VIEW_WIDTH: usize = 2 * VIEW_DIST_X + 1;
pub const VIEW_HEIGHT: usize = 2 * VIEW_DIST_Y + 1;

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("gem_miner", "ur dad")
        .window_setup(WindowSetup::default().title("Gem Miner"))
        .window_mode(
            WindowMode::default()
                .dimensions(800.0, 800.0)
                .resizable(true),
        )
        .build()
        .expect("could not create ggez context!");

    println!("Full filesystem info: {:#?}", ctx.fs);

    let g = GemMinerGame::new(&mut ctx).expect("could not build gem miner game");

    event::run(ctx, event_loop, g);
}

struct GemMinerGame {
    assets: Assets,
    state: State,
}

impl GemMinerGame {
    pub fn new(ctx: &mut Context) -> GameResult<GemMinerGame> {
        Ok(GemMinerGame {
            assets: Assets::load(ctx)?,
            state: State::new(),
        })
    }
}

impl EventHandler for GemMinerGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.state.update();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut canvas = Canvas::from_frame(ctx, CanvasLoadOp::Clear(Color::BLACK));

        // make pixel art good
        canvas.set_sampler(Sampler::nearest_clamp());

        self.state.draw(&mut canvas, &self.assets);

        // be done
        canvas.finish(ctx)
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: KeyInput,
        _repeated: bool,
    ) -> GameResult<()> {
        match input.keycode {
            Some(KeyCode::Up) => self.state.set_command(Command::Up),
            Some(KeyCode::Right) => self.state.set_command(Command::Right),
            Some(KeyCode::Down) => self.state.set_command(Command::Down),
            Some(KeyCode::Left) => self.state.set_command(Command::Left),
            Some(KeyCode::Backslash) => self.state.set_command(Command::PlaceSupport),
            Some(KeyCode::Escape) => event::request_quit(ctx),
            _ => (),
        }

        Ok(())
    }

    // fn key_down_event(
    //     &mut self,
    //     ctx: &mut Context,
    //     keycode: KeyCode,
    //     _keymods: event::KeyMods,
    //     _repeat: bool,
    // ) {
    //     match keycode {
    //         KeyCode::Up => self.state.set_command(Command::Up),
    //         KeyCode::Right => self.state.set_command(Command::Right),
    //         KeyCode::Down => self.state.set_command(Command::Down),
    //         KeyCode::Left => self.state.set_command(Command::Left),
    //         KeyCode::Escape => event::quit(ctx),
    //         _ => (),
    //     }
    // }
}
