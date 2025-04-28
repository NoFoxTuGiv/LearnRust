#![allow(dead_code)]
use macroquad::prelude::*;

mod paddle;
mod ball;

#[macroquad::main("Neon Pong")]
async fn main() {

    let mut ball = Ball::new()

    loop{
        clear_background(BLACK);

        ball.update_pos();
        ball.draw();

        next_frame().await;
    }
}
