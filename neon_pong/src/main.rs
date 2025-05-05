#![allow(unused)]
use ball::Ball;
use paddle::Paddle;
use macroquad::prelude::*;

mod ball;
mod paddle;

#[macroquad::main("Neon Pong")]
async fn main() {
    let mut ball = Ball::new(Vec2::new(screen_width() / 2., screen_height() / 2.), Vec2::splat(2.));

    let mut player = Paddle::new(Vec2::new(10., screen_height() / 2.));
    let mut ai = Paddle::new(Vec2::new(screen_width() - 20., screen_height() / 2.));

    println!("left paddle is player: {}", player.is_player());
    println!("right paddle is player: {}", ai.is_player());

    loop{
        clear_background(BLACK);

        ball.update_pos();
        ball.draw();

        player.show();
        player.update();
        ai.show();

        let paddles = [&player, &ai];
        ball.paddle_check(paddles);

        next_frame().await;
    }
}
