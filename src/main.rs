mod block;
mod loader;
mod map;
mod pos;
mod state;
mod world;

use game_loop::game_loop;
use loader::Assets;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use state::{Command, State};
use winit::{event::VirtualKeyCode, event_loop::EventLoop, window::WindowBuilder};
use winit_input_helper::WinitInputHelper;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 20;
pub const TILE_SIZE: usize = 8;

struct Game {
    input: WinitInputHelper,
    pixels: Pixels,
    state: State,
    assets: Assets,
}

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let game = Game {
        input: WinitInputHelper::new(),
        pixels: {
            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(
                (TILE_SIZE * WIDTH) as u32,
                (TILE_SIZE * HEIGHT) as u32,
                surface_texture,
            )?
        },
        state: State::new(),
        assets: Assets::load(),
    };

    game_loop(
        event_loop,
        window,
        game,
        5,
        0.5,
        |g| {
            g.game.state.update();
            println!("update");
            if g.game
                .pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                g.exit();
            }
        },
        |g| {
            // println!("draw");
            g.game.state.draw(g.game.pixels.get_frame(), &g.game.assets);
        },
        |g, event| {
            if g.game.input.update(event) {
                // close
                if g.game.input.key_pressed(VirtualKeyCode::Escape) || g.game.input.quit() {
                    g.exit();
                    return;
                }

                // input
                if g.game.input.key_pressed(VirtualKeyCode::Up) {
                    g.game.state.set_command(Command::Up)
                }

                if g.game.input.key_pressed(VirtualKeyCode::Right) {
                    g.game.state.set_command(Command::Right)
                }

                if g.game.input.key_pressed(VirtualKeyCode::Down) {
                    g.game.state.set_command(Command::Down)
                }

                if g.game.input.key_pressed(VirtualKeyCode::Left) {
                    g.game.state.set_command(Command::Left)
                }

                // resize
                if let Some(size) = g.game.input.window_resized() {
                    g.game.pixels.resize_surface(size.width, size.height);
                }
            }
        },
    );

    // event_loop.run(move |event, _, control_flow| {
    //     if let Event::RedrawRequested(_) = event {
    //         state.draw(pixels.get_frame(), &assets);

    //         if pixels.render().map_err(|e| e).is_err() {
    //             *control_flow = ControlFlow::Exit;
    //             return;
    //         }
    //     }

    //     if input.update(&event) {
    //         // close
    //         if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
    //             *control_flow = ControlFlow::Exit;
    //             return;
    //         }

    //         // input
    //         if input.key_held(VirtualKeyCode::Up) {
    //             state.set_command(game::Command::Up)
    //         }

    //         if input.key_held(VirtualKeyCode::Right) {
    //             state.set_command(game::Command::Right)
    //         }

    //         if input.key_held(VirtualKeyCode::Down) {
    //             state.set_command(game::Command::Down)
    //         }

    //         if input.key_held(VirtualKeyCode::Left) {
    //             state.set_command(game::Command::Left)
    //         }

    //         // resize
    //         if let Some(size) = input.window_resized() {
    //             pixels.resize_surface(size.width, size.height);
    //         }

    //         // draw
    //         window.request_redraw();

    //     }
    // });

    // Ok(())
}
