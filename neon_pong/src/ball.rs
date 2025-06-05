use crate::paddle::Paddle;
use macroquad::prelude::*;

/// Represents the game ball, which moves around the screen,
/// bounces off paddles and top/bottom walls, and resets when
/// it goes past the left or right edge.
pub struct Ball {
    /// Current position of the ball.
    pub pos: Vec2,
    /// Current velocity of the ball. The magnitude of this vector is 1.0;
    /// actual speed is applied via 'speed'.
    pub vel: Vec2,
    /// Radius in pixels
    r: f32,
    /// How fast the ball moves (pixels per second)
    speed: f32,
    /// Whether the ball has recently collided with a paddle (collision debounce)
    paddle_col: bool,
    /// Whether the ball has recently collided with a wall (collision debounce)
    wall_col: bool,
    /// Timestamp (in seconds) when the last paddle collision occured.
    p_col_time: f64,
    /// Timestamp (in seconds) when the last wall collision occured.
    w_col_time: f64,
}

impl Ball {
    /// Create a new `Ball` in the given position with the given velocity.
    ///
    /// # Arguments
    ///
    /// * `pos` – Initial `Vec2` position on screen (in pixels).
    /// * `vel` – Initial direction vector (should be a unit vector).
    ///
    /// # Returns
    ///
    /// A `Ball` with a fixed radius of 8.0 and speed of 200.0 px/s,
    /// ready to be updated/drawn.
    pub fn new(pos: Vec2, vel: Vec2) -> Ball {
        let r = 8.;
        let speed = 200.;

        Ball {
            pos,
            vel,
            r,
            speed,
            paddle_col: false,
            wall_col: false,
            p_col_time: 0.,
            w_col_time: 0.,
        }
    }

    /// Move the ball based on its velocity and handle top/bottom
    /// wall collisions as well as left/right scoring.
    ///
    /// # Arguments
    ///
    /// * `scores` – Mutable reference to a two‐element array holding
    ///   left‐side and right‐side scores. If the ball goes off the left
    ///   edge, increment `scores[0]`; if it goes off the right edge,
    ///   increment `scores[1]`.
    ///
    /// # Returns
    ///
    /// The updated `[usize; 2]` scores array after checking for out‐of‐bounds.
    ///
    /// # Behavior
    ///
    /// 1. Advances `pos` by `vel * speed * delta_time`.
    /// 2. If the ball hits the top (y ≤ 0) or bottom (y ≥ screen_height),
    ///    it reverses its `y` velocity and sets a short debounce timer
    ///    (`wall_col`) so it doesn’t bounce multiple times immediately.
    /// 3. If `pos.x < -75.`, the ball is considered “scored against” on
    ///    the left side: it recenters to the screen midpoint and
    ///    increments `scores[0]`.
    /// 4. If `pos.x > screen_width() + 75.`, the ball is scored against
    ///    on the right side: it recenters and increments `scores[1]`.
    /// 5. Otherwise, returns the unmodified scores.
    pub fn update_pos(&mut self, scores: &mut [usize; 2]) -> [usize; 2] {
        let d_time = get_frame_time();

        // Move the ball
        self.pos += self.vel * d_time * self.speed;

        // If we recently collided with a wall, wait 0.2 seconds (Just over 1 frame on average) before
        // allowing another collision.
        if self.wall_col && get_time() > self.w_col_time + 0.2 {
            self.wall_col = false;
        }

        // Check for new top/bottom collision
        if !self.wall_col && (self.pos.y >= screen_height() || self.pos.y <= 0.) {
            self.vel.y *= -1.;
            self.wall_col = true;
            self.w_col_time = get_time();
        }

        // Check left/right out-of-bounds for scoring and reset to center if scored
        // TODO: New Round Logic @ Score Cap
        if self.pos.x < -75. {
            self.pos = Vec2::new(screen_width() / 2., screen_height() / 2.);
            scores[0] += 1;
            *scores
        } else if self.pos.x > screen_width() + 75. {
            self.pos = Vec2::new(screen_width() / 2., screen_height() / 2.);
            scores[1] += 1;
            *scores
        } else {
            *scores
        }
    }

    /// Check for collisions against both paddles and bounce horizontally if needed.
    ///
    /// # Arguments
    ///
    /// * `paddles` – An array containing references to the two `Paddle` objects.
    ///
    /// # Behavior
    ///
    /// 1. Debounces paddle collisions for 0.2 seconds using `paddle_col`.
    /// 2. Calls the private `is_colliding` helper to see if the ball overlaps
    ///    either paddle’s rectangle.
    /// 3. If a new collision is detected, reverse the `x` velocity (`vel.x *= -1`)
    ///    and set the `paddle_col` flag with the current time to avoid repeated
    ///    bounces instantly.
    pub fn paddle_check(&mut self, paddles: [&Paddle; 2]) {
        // If we recently collided with a paddle, wait 0.2 seconds before new collision
        if self.paddle_col && get_time() > self.p_col_time + 0.2 {
            self.paddle_col = false;
        }

        // If not currently debounced, and we detect a collision
        if !self.paddle_col && self.is_colliding(paddles) {
            self.vel.x *= -1.;              // Reverse horizontal direction
            self.paddle_col = true;         // Set debounce flag
            self.p_col_time = get_time();   // Set debounce timer
        }
    }

    /// Draw the ball as a filled white circle at its current position.
    ///
    /// # Behavior
    ///
    /// Uses `macroquad::draw_circle` with the ball’s `pos` and radius `r`.
    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.r, WHITE);
    }

    /// Check if the ball’s bounding circle overlaps either paddle’s rectangle.
    ///
    /// # Arguments
    ///
    /// * `paddles` – An array of two `&Paddle` references.
    ///
    /// # Returns
    ///
    /// `true` if the ball is currently overlapping either paddle, `false` otherwise.
    ///
    /// # Implementation Details
    ///
    /// 1. Computes the ball’s top (`b_top`), bottom (`b_bot`), left (`b_l`),
    ///    and right (`b_r`) edges based on its center `pos` and radius `r`.
    /// 2. For each paddle, retrieves its top edge (`p_top`), bottom edge (`p_bot`),
    ///    left edge (`p_l`), and right edge (`p_r`), based on the paddle’s
    ///    `pos` plus its `get_height()` and `get_width()`.
    /// 3. Uses AABB‐circle overlap logic: if the ball’s vertical span overlaps
    ///    `[p_top..p_bot]` *and* horizontal span overlaps `[p_l..p_r]`, there is a collision.
    fn is_colliding(&self, paddles: [&Paddle; 2]) -> bool {
        let b_top = self.pos.y - self.r;
        let b_bot = self.pos.y + self.r;
        let b_l = self.pos.x - self.r;
        let b_r = self.pos.x + self.r;

        for paddle in paddles {
            let p_top = paddle.pos.y;
            let p_bot = paddle.pos.y + paddle.get_height();
            let p_l = paddle.pos.x;
            let p_r = paddle.pos.x + paddle.get_width();

            if b_bot >= p_top && b_top <= p_bot && b_r >= p_l && b_l <= p_r {
                return true;
            }
        }

        false
    }
}
