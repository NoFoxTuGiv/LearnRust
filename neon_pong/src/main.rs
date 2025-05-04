#![allow(unused)]
use crate::ball::Ball;
use macroquad::prelude::*;

mod paddle;
mod ball;

#[macroquad::main("Neon Pong")]
async fn main() {
    let mut ball = Ball::new(Vec2::new(screen_width() / 2., screen_height() / 2.), Vec2::splat(2.));

    loop{
        clear_background(BLACK);


        ball.update_pos();
        ball.draw();

        next_frame().await;
    }
}
