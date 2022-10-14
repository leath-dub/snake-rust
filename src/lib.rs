use std::collections::LinkedList;

pub struct SdlWrapper {
    pub context: sdl2::Sdl,
    pub window: sdl2::video::Window
}

pub struct Game;
pub const NAME: &str = "Snake";
pub const SIZE: u32 = 10;

impl Game {
    /* initialize what we need from sdl */
    pub fn init_sdl() -> SdlWrapper {
        let context = sdl2::init().unwrap();
        let video_subsystem = context.video().unwrap();
        let window = video_subsystem
            .window(NAME, 800, 600)
            .position_centered()
            .build()
            .unwrap();
        SdlWrapper { context: context, window: window }
    }
}

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

pub struct Snake {
    body: LinkedList<Chode>,
    head: Chode
}

impl Snake {
    pub fn new(pos: [i32; 2], vel: [i32; 2]) -> Snake {
        Snake {
            body: LinkedList::new(),
            head: Chode {
                pos: pos,
                vel: vel
            }
        }
    }
    pub fn push_back(&mut self, other: Chode) {
        self.push_back(other);
    }
}
