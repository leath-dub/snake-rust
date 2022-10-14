use snake_game::*;
use std::io;

pub fn main() {
    let mut ctx = Game::init_sdl();

    /* convert the window context into a canvas(one we can draw on) */
    let mut cvs = ctx.window
        .into_canvas()
        .build()
        .unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input);

    /* snake part */
    let mut snake: Snake = Snake::new([0, 0], [1, 0]);
    snake.push_back(Chode::new([SIZE as i32, 0], [1, 0]));
}