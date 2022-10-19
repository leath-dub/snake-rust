extern crate sdl2;
extern crate rand;

use snake_game::*;

/* sdl2 stuff */
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use sdl2::gfx::framerate::FPSManager;

pub fn main() {
    let game = Game::init_sdl();

    /* convert the window context into a canvas(one we can draw on) */
    let mut cvs = game.window
        .into_canvas()
        .build()
        .unwrap();

    /* fruit part */
    let mut fruit: Fruit = Fruit::new();

    let mut snake: Snake = Snake::new(rand_pos(), [1, 0]);
    snake.make_body(4);

    let mut fps = FPSManager::new();
    fps.set_framerate(10).ok();
    let mut count = 0;

    let mut ev_pmp = game.context.event_pump().unwrap();
    'running: loop {
        if snake.end() {
            break 'running;
        }
        cvs.set_draw_color(Color::RGB(168, 193, 129));
        cvs.clear();
        draw_fruit(&fruit, &mut cvs);
        for ev in ev_pmp.poll_iter() {
             match ev {
                 Event::KeyDown {keycode: Some(Keycode::H), ..} |
                 Event::KeyDown {keycode: Some(Keycode::Left), ..} => {
                     snake.change_velocity([-1, 0]);
                 },
                 Event::KeyDown {keycode: Some(Keycode::J), ..} |
                 Event::KeyDown {keycode: Some(Keycode::Down), ..} => {
                     snake.change_velocity([0, 1]);
                 },
                 Event::KeyDown {keycode: Some(Keycode::K), ..} |
                 Event::KeyDown {keycode: Some(Keycode::Up), ..} => {
                     snake.change_velocity([0, -1]);
                 },
                 Event::KeyDown {keycode: Some(Keycode::L), ..} |
                 Event::KeyDown {keycode: Some(Keycode::Right), ..} => {
                     snake.change_velocity([1, 0]);
                 },
                 Event::Quit {..} |
                 Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                     break 'running
                 },
                 _ => ()
             }
        }
        snake.slither(&None);
        snake.nom(&mut fruit);
        cvs.set_draw_color(Color::RGB(141, 161, 1));
        draw_snake(&snake, &mut cvs);
        cvs.present();
        fps.delay();
    }
}
