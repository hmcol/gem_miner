use crate::{
    block::{Block, Ore},
    game::Game,
    pos::Coord,
    HEIGHT, WIDTH,
};
use crossterm::{
    cursor::{Hide, MoveTo},
    event::{poll, read, Event, KeyCode},
    execute,
    style::{Color, StyledContent, Stylize},
    terminal::{Clear, ClearType},
};
use std::io::stdout;

const BROWN: Color = Color::AnsiValue(94);
const BACKGROUND: Color = Color::Black;
const BORDER: Color = Color::DarkMagenta;

const RADIUS: usize = 4;

pub fn init() {
    execute!(stdout(), Clear(ClearType::All), Hide).expect("failed to clear console");
}

pub fn frame(g: &Game) {
    execute!(stdout(), MoveTo(0, 0)).expect("failed resetting cursor");

    println!("{}", format!("┌{0}┬{0}┐", "─".repeat(RADIUS)).with(BORDER));

    let cx = g.miner.x.clamp(RADIUS, WIDTH - 1 - RADIUS) - RADIUS;
    let cy = g.miner.y.clamp(RADIUS, HEIGHT - 1 - RADIUS) - RADIUS;

    for dy in 0..(2 * RADIUS + 1) {

        if dy == RADIUS {
            print!("{}", '├'.with(BORDER));
        } else {
            print!("{}", '│'.with(BORDER));
        }

        
        for dx in 0..(2 * RADIUS + 1) {
            let pos = Coord::new(cx + dx, cy + dy);

            if pos == g.miner {
                //☺☻
                print!("{}", '@'.on_black());
            } else if g.support.get(pos) == Some(&true) {
                if g.block.get(pos) == Some(&Block::Ladder) {
                    print!("{}", '╫'.with(Color::AnsiValue(94)));
                } else {
                    print!("{}", '|'.with(Color::AnsiValue(94)));
                }
            } else if let Some(b) = g.block.get(pos) {
                print!("{}", b.tile());
            } else {
                print!("{}", '?'.red().on_white());
            }
        }
        if dy == RADIUS {
            println!("{}", '┤'.with(BORDER));
        } else {
            println!("{}", '│'.with(BORDER));
        }
    }

    println!("{}", format!("└{0}┴{0}┘", "─".repeat(RADIUS)).with(BORDER));

    println!();

    // print!("Digging: ");

    // if let State::Dig(dir) = g.state {
    //     if let Block::Dirt(_, dmg) = g.get_block_offset(dir) {
    //         print!("{}", "X".repeat(1 + dmg as usize))
    //     }
    // }
    // print!("{}", " ".repeat(10));
}

trait Tile {
    fn tile(self) -> StyledContent<char>;
}

impl Tile for Block {
    fn tile(self) -> StyledContent<char> {
        match self {
            Block::Air => ' '.on(BACKGROUND),
            Block::Dirt(None, dmg) => match dmg {
                0 => '░',
                1 => '▒',
                2 => '▓',
                3 => '█',
                _ => '?',
            }
            .dark_grey(),
            Block::Dirt(Some(ore), _) => match ore {
                Ore::Coal => '*'.black(),
                Ore::Iron => '≈'.dark_yellow(),
                Ore::GreenOpal => '♣'.green(),
                Ore::WhiteOpal => '♣'.white(),
                Ore::Silver => '≈'.grey(),
                Ore::Gold => '≈'.yellow(),
                Ore::Ruby => '♥'.red(),
                Ore::RedOpal => '♣'.red(),
                Ore::Emerald => '♥'.green(),
                Ore::BlackOpal => '♣'.black(),
                Ore::Sapphire => '♥'.blue(),
                Ore::Diamond => '♦'.cyan(),
                Ore::Uranium => 'α'.green(),
                Ore::Platinum => '♠'.white(),
            }
            .on_dark_grey(),
            Block::Stone(_) => '○'.grey().on_dark_grey(),
            Block::Ladder => '#'.with(BROWN).on(BACKGROUND),
        }
    }
}
