//! Neon Pong: a simple Pong clone built with Macroquad.
//!
//! This module sets up the game window, initializes the ball and paddles,
//! and drives the main game loop. During each frame, it:
//! 1. Clears the screen to black.
//! 2. Updates and draws the ball (including wall‐bounce and scoring).
//! 3. Renders and moves the player paddle based on “W”/“S” input.
//! 4. Renders and moves the AI paddle to track the ball’s Y position.
//! 5. Checks for ball–paddle collisions and bounces the ball.
//! 6. Draws the current scores in the top‐left and top‐right corners.
//! 7. Exits cleanly if the Escape key is pressed.
//! 8. Awaits the next frame via `next_frame().await`.

use ball::Ball;
use macroquad::prelude::*;
use paddle::Paddle;

mod ball;
mod paddle;

/// Entry point for the Neon Pong game.
///
/// This function:
/// 1. Creates a `Ball` centered on screen with an initial velocity of `Vec2::splat(2.)`.
/// 2. Creates two `Paddle` instances:
///    - The player paddle at x = 10.0.
///    - The AI paddle at x = screen_width() – 20.0.
/// 3. Initializes a two‐element score array `[0, 0]`.
///
/// Then enters a perpetual loop (the game loop) and, each frame:
/// - Clears the background to `BLACK`.
/// - Calls `ball.update_pos(&mut scores)` to move the ball, handle wall bounces,
///   and automatically reset & increment scores if the ball goes off‐screen.
/// - Draws the ball via `ball.draw()`.
/// - Draws and updates the player paddle:
///     • `player.show()` renders the rectangle.
///     • `player.update()` moves it if “W” or “S” is pressed and clamps it inside the screen.
/// - Draws and moves the AI paddle:
///     • `ai.show()` renders the rectangle.
///     • `ai.ai_move(ball_pos)` moves it up/down to follow the ball’s Y coordinate, then clamps.
/// - Bundles both paddles into `let paddles = [&player, &ai];` and calls
///   `ball.paddle_check(paddles)` to detect any collisions and bounce the ball horizontally.
/// - Renders the scores:
///     • Left score (`scores[0]`) at coordinates (25.0, 45.0), font size 55.
///     • Right score (`scores[1]`) at (screen_width() – 65.0, 45.0), font size 55.
/// - If the Escape key is pressed (`is_key_pressed(KeyCode::Escape)`), breaks out of the loop and exits.
/// - Otherwise, calls `next_frame().await` to wait until Macroquad is ready for the next frame.
///
/// # Attributes
///
/// - `#[macroquad::main("Neon Pong")]` sets up a window titled “Neon Pong”
///   and invokes this `async fn main()` as Macroquad’s startup.
#[macroquad::main("Neon Pong")]
async fn main() {
    // 1. Initialize the ball at the center of the screen with a small initial velocity.
    let mut ball = Ball::new(
        Vec2::new(screen_width() / 2., screen_height() / 2.),
        Vec2::splat(2.),
    );

    // 2. Initialize the player paddle (left side) and AI paddle (right side).
    let mut player = Paddle::new(Vec2::new(10., screen_height() / 2.));
    let mut ai = Paddle::new(Vec2::new(screen_width() - 20., screen_height() / 2.));

    // 3. Scores: [player_score, ai_score], both start at 0.
    let mut scores: [usize; 2] = [0; 2];

    // === Game Loop ===
    loop {
        // Clear the background
        clear_background(BLACK);

        // === Draw mid-line ===
        (5..screen_height() as i32).step_by(20).for_each(|y| {
            draw_line(
                screen_width() / 2. - 2.,
                y as f32,
                screen_width() / 2. - 2.,
                (y + 10) as f32,
                4.,
                LIGHTGRAY,
            )
        });

        // === Ball update & draw ===
        // Move the ball, handle top/bottom bounces, and reset + increment scores if it goes off‐screen.
        scores = ball.update_pos(&mut scores);
        // Draw the ball as a white circle.
        ball.draw();

        // === Player paddle ===
        // Draw the player paddle rectangle.
        player.show();
        // Move it if the “W” or “S” key is held (and clamp to screen edges).
        player.update();

        // === AI paddle ===
        // Draw the AI paddle rectangle.
        ai.show();
        // Move AI paddle toward the ball’s current Y position (and clamp).
        let ball_pos = ball.pos;
        ai.ai_move(ball_pos);

        // === Ball–Paddle collision ===
        // Bundle both paddles into an array of references.
        let paddles = [&player, &ai];
        // If the ball overlaps a paddle, reverse ball.vel.x and debounce.
        ball.paddle_check(paddles);

        // === Draw scores ===
        // Left player’s score in top-left corner.
        draw_text(&scores[0].to_string(), 25., 45., 55., WHITE);
        // AI's score in the top-right corner.
        draw_text(
            &scores[1].to_string(),
            screen_width() - 65.,
            45.,
            55.,
            WHITE,
        );

        // === Exit condition ===
        // If the user presses Escape, break out of the loop and terminate.
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        // Wait for the next frame before repeating.
        next_frame().await;
    }
}
