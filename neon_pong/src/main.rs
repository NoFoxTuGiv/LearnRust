use macroquad::prelude::*;
use ball::Ball;
use paddle::Paddle;

mod ball;
mod paddle;

#[macroquad::main("Neon Pong")]
async fn main() {
    let mut ball = Ball::new(
        Vec2::new(screen_width() / 2., screen_height() / 2.),
        Vec2::splat(2.),
    );

    let mut player = Paddle::new(Vec2::new(10., screen_height() / 2.));
    let mut ai = Paddle::new(Vec2::new(screen_width() - 20., screen_height() / 2.));

    let mut scores: [usize; 2] = [0; 2];

    loop {
        clear_background(BLACK);

        scores = ball.update_pos(&mut scores);
        ball.draw();

        player.show();
        player.update();
        ai.show();
        let ball_pos = ball.pos;
        ai.ai_move(ball_pos);

        let paddles = [&player, &ai];
        ball.paddle_check(paddles);

        draw_text(&scores[0].to_string(), 25., 45., 55., WHITE);
        draw_text(&scores[1].to_string(), screen_width() - 65., 45., 55., WHITE);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
