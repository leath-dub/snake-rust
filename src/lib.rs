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

struct Chode {
    pos: [i32; 2],
    vel: [i32; 2],
}

struct Snake {
    body: LinkedList<Segment>,
    head: Chode
}

impl Snake {
    fn new(pos: [i32; 2], vel: [i32; 2]) -> Snake {
        Snake {
            Snake::body: LinkedList::new() as Chode,
            Snake::head: Chode {
                pos: [pos[0], pos[1]],
                vel: [vel[0], vel[1]]
            }
        }
    }
    fn append(&mut self, other: &mut LinkedLis
}
