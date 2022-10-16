use std::collections::LinkedList;
use std::collections::linked_list::IterMut;

pub struct SdlWrapper {
    pub context: sdl2::Sdl,
    pub window: sdl2::video::Window
}

pub struct Game;
pub const NAME: &str = "Snake";
pub const SIZE: u32 = 40;
pub const GAME_WIDTH: u32 = 600;
pub const GAME_HEIGHT: u32 = 600;

impl Game {
    /* initialize what we need from sdl */
    pub fn init_sdl() -> SdlWrapper {
        let context = sdl2::init().unwrap();
        let video_subsystem = context.video().unwrap();
        let window = video_subsystem
            .window(NAME, GAME_WIDTH, GAME_HEIGHT)
            .position_centered()
            .build()
            .unwrap();
        SdlWrapper { context: context, window: window }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Cardinal {
    North,
    South,
    East,
    West
}

#[derive(Debug)]
pub struct Chode {
    pos: [i32; 2],
    vel: [i32; 2],
}

impl Chode {
    pub fn new(pos: [i32; 2], vel: [i32; 2]) -> Self {
        Chode {
            pos: pos,
            vel: vel
        }
    }
}

#[derive(Debug)]
pub struct Snake {
    body: LinkedList<Chode>,
    head: Chode
}

#[derive(Debug)]
pub struct Fruit {
    pos: [i32; 2]
}

impl Fruit {
    pub fn new() -> Fruit {
        Fruit {pos: Self::rand_pos()}
    }
    pub fn update(&mut self) {
        self.pos = Self::rand_pos();
    }
    fn rand_pos() -> [i32; 2] {
        [
            ((rand::random::<u32>() % (GAME_WIDTH - SIZE) / SIZE) * SIZE) as i32,
            ((rand::random::<u32>() % (GAME_HEIGHT - SIZE) / SIZE) * SIZE) as i32
        ]
    }
}

impl Snake {
    pub fn make_body(&mut self, len: u32) -> Option<()> {
        let direction: Cardinal = match vel2dir(
            /* reverse of heads velocity */
            &[-self.head.vel[0], -self.head.vel[1]]
        ) {
            None => return None,
            Some(direction) => direction
        };

        for n in 1..(len + 1) {
            self.push_back(Chode::new(
                Self::move_chode(&self.head, direction, (n * SIZE) as i32)
            , self.head.vel));
        }

        return Some(());
    }
    pub fn new(pos: [i32; 2], vel: [i32; 2]) -> Snake {
        Snake {
            body: LinkedList::new(),
            head: Chode {
                pos: pos,
                vel: vel
            }
        }
    }
    pub fn change_velocity(&mut self, vel: [i32; 2]) {
        self.head.vel = vel;
    }
    pub fn slither(&mut self, opt: &Option<[i32; 2]>) {
        let vel: &[i32; 2] = match opt {
            /* if a new velocity vector is provided,
             * we should update the snakes velocity vector,
             * and move based on that
             */
            Some(x) => {
                self.head.vel = [x[0], x[1]];
                x
            },
            None => &self.head.vel
        };
        /* move the head */
        self.head.pos = Self::move_chode(
            &self.head, match vel2dir(&self.head.vel) {
                Some(direction) => direction,
                None => Cardinal::North // TODO: propagate an error
            }, SIZE as i32);
        /* move the children( the body ) */
        Self::update_neighbours(self.body.iter_mut(), &vel);
    }
    pub fn nom(&mut self, fruit: &mut Fruit) {
        if Self::collides(&fruit.pos, &self.head.pos) {
            fruit.update();
            self.push_back(Chode::new(match self.body.back() {
                Some(tail) => Self::move_chode(tail, match vel2dir(&[-tail.vel[0], -tail.vel[1]]) {
                        Some(direction) => direction,
                        None => Cardinal::North
                    }, SIZE as i32),
                None => [-1, 0]
            }, self.body.back().unwrap().vel));
        }
    }
    fn collides(a: &[i32; 2], b: &[i32; 2]) -> bool {
        a[0] == b[0] && a[1] == b[1]
    }
    pub fn end(&self) -> bool {
        for chode in self.body.iter() {
            if Self::collides(&self.head.pos, &chode.pos) {
                return true;
            }
        }
        false
    }
    fn update_neighbours(mut iter: IterMut<Chode>, vel: &[i32; 2]) {
        match iter.next() {
            Some(chode) => {
                Self::update_neighbours(iter, &chode.vel);
                chode.pos = Self::move_chode(
                    &chode, match vel2dir(&chode.vel) {
                        Some(x) => x,
                        None => Cardinal::North // TODO: propagate an error
                    }, SIZE as i32
                );
                chode.vel = [vel[0], vel[1]]; // update velocity to parents
            },
            None => ()
        };
    }
    fn move_chode(chode: &Chode, direction: Cardinal, dist: i32) -> [i32; 2] {
        match direction {
            /* TODO modulus of negative not working */
            Cardinal::North => [chode.pos[0], (chode.pos[1] - dist) % GAME_WIDTH as i32],
            Cardinal::South => [chode.pos[0], (chode.pos[1] + dist) % GAME_WIDTH as i32],
            Cardinal::East => [(chode.pos[0] + dist) % GAME_WIDTH as i32, chode.pos[1]],
            Cardinal::West => [(chode.pos[0] - dist) % GAME_WIDTH as i32, chode.pos[1]]
        }
    }
    fn push_back(&mut self, other: Chode) {
        self.body.push_back(other);
    }
}

pub fn vel2dir(&vel: &[i32; 2]) -> Option<Cardinal> {
    if vel[0] * vel[1] != 0 || vel[0] + vel[1] == 0 {
        return None;
    }
    match vel[0] {
        0 => match vel[1] > 0 {
            true => Some(Cardinal::South),
            false => Some(Cardinal::North)
        },
        _ => match vel[0] > 0 {
            true => Some(Cardinal::East),
            false => Some(Cardinal::West)
        }
    }
}

pub fn draw_snake(snake: &Snake, game: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    game.set_draw_color(sdl2::pixels::Color::RGB(141, 161, 1));
    /* draw the head */
    draw_rect(sdl2::rect::Rect::new(
        snake.head.pos[0], snake.head.pos[1], SIZE, SIZE
    ), game);
    /* draw the body */
    for chode in snake.body.iter() {
        draw_rect(sdl2::rect::Rect::new(
            chode.pos[0], chode.pos[1], SIZE, SIZE
        ), game);
    }
}

pub fn draw_fruit(fruit: &Fruit, game: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    game.set_draw_color(sdl2::pixels::Color::RGB(230, 126, 128));
    draw_rect(sdl2::rect::Rect::new(
        fruit.pos[0], fruit.pos[1], SIZE, SIZE
    ), game);
}

fn draw_rect(rect: sdl2::rect::Rect,game: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    game.fill_rect(rect);
}
