
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

struct Game {
    blocks: [[Block; WIDTH]; HEIGHT],
    supports: [[bool; WIDTH]; HEIGHT],
    pos: Coord,
    state: State,
}

impl Game {
    pub fn new() -> Self {
        Game {
            blocks: gen::new_map(),
            supports: [[false; WIDTH]; HEIGHT],
            pos: Coord { x: 0, y: 0 },
            state: State::Idle,
        }
    }

    fn draw(&self) {
        let y_min = self.pos.y.saturating_sub(4);
        let y_max = (self.pos.y + 4).min(HEIGHT);
        let x_min = self.pos.x.saturating_sub(4);
        let x_max = (self.pos.x + 4).min(WIDTH);

        for y in y_min..y_max {
            for x in x_min..x_max {
                if x == self.pos.x && y == self.pos.y {
                    //☺☻
                    print!("{}", '@'.on_black());
                } else if self.supports[y][x] {
                    if self.blocks[y][x] == Block::Ladder {
                        print!("{}", '╫'.with(Color::AnsiValue(94)));
                    } else {
                        print!("{}", '|'.with(Color::AnsiValue(94)));
                    }
                } else {
                    print!("{}", self.blocks[y][x].tile());
                }
            }
            println!();
        }

        println!();

        print!("Digging: ");

        if let State::Dig(dir) = self.state {
            if let Block::Dirt(_, dmg) = self.get_block_offset(dir) {
                print!("{}", "X".repeat(1 + dmg as usize))
            }
        }
        print!("{}", " ".repeat(10));
    }

    fn update(&mut self, cmd: Command) {
        let (x, y) = (self.pos.x, self.pos.y);

        match self.state {
            State::Idle => match cmd {
                Command::Dir(dir) => self.handle_action(dir),
                Command::PlaceLadder => {
                    if self.get_cur_block() != Block::Ladder {
                        self.blocks[y][x] = Block::Ladder;
                        self.try_move(Direction::North);
                    }
                }
                Command::PlaceSNorthport => self.place_support(self.pos),
                _ => (),
            },
            State::Dig(dir) => {
                if let Command::Dir(new_dir) = cmd {
                    // self.handle_action(new_dir);
                    if new_dir != dir {
                        self.handle_action(new_dir);
                        return;
                    }
                }
                if !self.try_dig(dir) {
                    self.state = State::Idle;
                }

                if self.get_block_offset(Direction::South) == Block::Air {
                    self.state = State::Fall;
                }
            }
            State::Fall => {
                if self.blocks[y + 1][x] == Block::Air {
                    self.pos.y += 1;
                } else {
                    self.state = State::Idle;
                }
            }
        }

        for y in 0..(HEIGHT - 1) {
            for x in 0..WIDTH {
                if self.blocks[y][x].is_fall()
                    && self.blocks[y + 1][x] == Block::Air
                    && !self.supports[y + 1][x]
                {
                    self.blocks[y + 1][x] = self.blocks[y][x];
                    self.blocks[y][x] = Block::Air;
                }
            }
        }
    }

    fn handle_action(&mut self, dir: Direction) {
        if self.try_move(dir) {
            self.state = State::Idle;
        } else if self.try_dig(dir) {
            self.state = State::Dig(dir);
        } else {
            self.state = State::Idle;
        }

        if self.get_block_offset(Direction::South) == Block::Air {
            self.state = State::Fall
        }
    }

    fn get_block(&self, coord: Coord) -> Block {
        self.blocks[coord.y][coord.x]
    }

    fn get_cur_block(&self) -> Block {
        self.get_block(self.pos)
    }

    fn get_block_offset(&self, dir: Direction) -> Block {
        self.get_block(self.pos.offset(dir))
    }

    fn try_move(&mut self, dir: Direction) -> bool {
        if !self.get_block_offset(dir).is_open()
            || dir == Direction::North && self.get_cur_block() != Block::Ladder
        {
            return false;
        }

        // if dir == Direction::North && self.get_cur_block() != Block::Ladder {
        //     self.blocks[self.pos.y][self.pos.x] = Block::Ladder;
        // }

        self.pos = self.pos.offset(dir);
        true
    }

    fn try_dig(&mut self, dir: Direction) -> bool {
        let t = self.pos.offset(dir);

        if let Block::Dirt(_, dmg) = &mut self.blocks[t.y][t.x] {
            if *dmg > 0 {
                *dmg -= 1;
            } else {
                self.blocks[t.y][t.x] = Block::Air;

                if self.supports[t.y - 1][t.x] || self.supports[t.y + 1][t.x] {
                    self.place_support(t)
                }
            }
            true
        } else {
            false
        }
    }

    fn place_support(&mut self, coord: Coord) {
        let (x, y) = (coord.x, coord.y);

        let mut dy = 0;
        while y + dy < HEIGHT && self.blocks[y + dy][x].is_open() && !self.supports[y + dy][x] {
            self.supports[y + dy][x] = true;
            dy += 1;
        }

        dy = 1;
        while y > dy && self.blocks[y - dy][x].is_open() && !self.supports[y - dy][x] {
            self.supports[y - dy][x] = true;
            dy += 1;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Idle,
    Dig(Direction),
    Fall,
}

#[derive(Debug)]
struct Stats {
    energy_max: u8,
    lantern_level: u8,
    bag_level: u8,
    pick_level: u8,
    drill_level: u8,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            energy_max: 100,
            lantern_level: 0,
            bag_level: 0,
            pick_level: 0,
            drill_level: 0,
        }
    }
}
