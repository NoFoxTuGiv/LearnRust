use macroquad::prelude::*;

/// The base movement speed of a paddle, in pixels per second.
/// This value is multiplied by `delta_time` each frame to determine
/// how far the paddle moves per update.
const BASE_SPEED: f32 = 500.;

/// Represents a paddle. Each paddle has a rectangular shape, a position, and is either controlled by the
/// player (left side) or by simple AI logic (right side).
pub struct Paddle {
    /// Top‐left corner position of the paddle on screen (in pixels).
    pub pos: Vec2,
    /// Vertical size of the paddle (in pixels).
    height: f32,
    /// Horizontal size of the paddle (in pixels).
    width: f32,
    /// `true` if this paddle is controlled by the human player (i.e.,
    /// placed on the left half of the screen), or `false` if it’s AI‐controlled.
    player: bool,
}

impl Paddle {
    /// Create a new `Paddle` at the given screen position.
    ///
    /// # Arguments
    ///
    /// * `pos` – A `Vec2` specifying the top‐left corner of the paddle. If
    ///   `pos.x` is less than half of `screen_width()`, this paddle will
    ///   be flagged as the player’s paddle; otherwise, it becomes the AI paddle.
    ///
    /// # Returns
    ///
    /// A `Paddle` with a fixed width of 10.0 px and height of 80.0 px.
    /// The `player` flag is automatically determined from `pos.x`.
    pub fn new(pos: Vec2) -> Paddle {
        // Determine if this paddle is on the left half of the screen
        let player = pos.x < screen_width() / 2.;
        let height: f32 = 80.;
        let width: f32 = 10.;
        Paddle {
            pos,
            height,
            width,
            player,
        }
    }

    /// Get this paddle’s height in pixels.
    ///
    /// # Returns
    ///
    /// A `f32` equal to the paddle’s vertical size (e.g., 80.0).
    pub fn get_height(&self) -> f32 {
        self.height
    }

    /// Get this paddle’s width in pixels.
    ///
    /// # Returns
    ///
    /// A `f32` equal to the paddle’s horizontal size (e.g., 10.0).
    pub fn get_width(&self) -> f32 {
        self.width
    }

    /// Indicates whether this paddle is controlled by the player.
    ///
    /// # Returns
    ///
    /// `true` if the paddle was placed on the left half of the screen
    /// (player’s paddle), or `false` if it’s the AI paddle.
    #[allow(unused)]
    pub fn is_player(&self) -> bool {
        self.player
    }

    /// Update the paddle’s position when it’s controlled by the human player.
    ///
    /// # Behavior
    ///
    /// 1. Computes `move_speed = BASE_SPEED * delta_time`.  
    /// 2. If the “W” key is held down and this paddle’s `player` flag is `true`,
    ///    move it upward by `move_speed` (subtract from `pos.y`).  
    /// 3. If the “S” key is held down and `player` is `true`, move it downward
    ///    by `move_speed` (add to `pos.y`).  
    /// 4. Clamp the paddle’s `y` position within [0, screen_height() – height]
    ///    by calling the private `edges()` method.
    pub fn update(&mut self) {
        let delta_time = get_frame_time();
        let move_speed = BASE_SPEED * delta_time;

        if self.player & is_key_down(KeyCode::W) {
            self.pos -= Vec2::new(0., move_speed);
        }
        if self.player & is_key_down(KeyCode::S) {
            self.pos += Vec2::new(0., move_speed);
        }
        self.edges();
    }

    /// Simple AI logic to follow the ball’s vertical position.
    ///
    /// # Arguments
    ///
    /// * `b_pos` – A `Vec2` representing the current center position of the ball.
    ///
    /// # Behavior
    ///
    /// 1. Computes `move_speed = BASE_SPEED * delta_time`.  
    /// 2. If the paddle’s vertical center (`pos.y + height/2`) is below `b_pos.y`,
    ///    move the paddle down by `move_speed`.  
    /// 3. If the paddle’s vertical center is above `b_pos.y`, move the paddle up
    ///    by `move_speed`.  
    /// 4. After movement, clamp `pos.y` within [0, screen_height() – height] using `edges()`.
    pub fn ai_move(&mut self, b_pos: Vec2) {
        let delta_time = get_frame_time();
        let move_speed = BASE_SPEED * delta_time;

        if self.pos.y + self.height / 2. < b_pos.y {
            self.pos += Vec2::new(0., move_speed);
        }
        if self.pos.y + self.height / 2. > b_pos.y {
            self.pos -= Vec2::new(0., move_speed);
        }
        self.edges();
    }

    /// Ensure the paddle remains fully on screen by clamping its `y` position.
    ///
    /// # Behavior
    ///
    /// - If `pos.y > screen_height() – height`, set `pos.y = screen_height() – height`.  
    /// - If `pos.y < 0.0`, set `pos.y = 0.0`.  
    ///
    /// This prevents the paddle from moving off the top or bottom edge of the window.
    fn edges(&mut self) {
        if self.pos.y > screen_height() - self.height {
            self.pos.y = screen_height() - self.height;
        }
        if self.pos.y < 0. {
            self.pos.y = 0.;
        }
    }

    /// Draw the paddle as a solid white rectangle at its current `pos`.
    ///
    /// # Behavior
    ///
    /// Uses `macroquad::draw_rectangle(x, y, width, height, WHITE)` where:
    /// - `(x, y)` is the paddle’s top‐left corner (`self.pos`),
    /// - `(width, height)` is `(self.width, self.height)`.
    pub fn show(&self) {
        draw_rectangle(self.pos.x, self.pos.y, self.width, self.height, WHITE);
    }
}
