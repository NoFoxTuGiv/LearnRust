use macroquad::prelude::*;
use std::f32::consts::{PI, TAU};

#[macroquad::main("Baton")]
async fn main() {
    let mut baton = Baton::new();

    loop {
        clear_background(DARKGRAY);

        baton.draw();
        baton.update();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}

struct Baton {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    angle: f32,
    angle_v: f32,
    prev_m_angle: f32,
    is_dragging: bool,
}

impl Baton {
    /// Creates a new Baton object with no current velocity.
    fn new() -> Baton {
        let x = screen_width() / 2.;
        let y = screen_height() / 2.;
        let w = 200.;
        let h = 15.;

        Baton {
            x,
            y,
            w,
            h,
            angle: 0.,
            angle_v: 0.,
            prev_m_angle: 0.,
            is_dragging: false,
        }
    }

    /// Updates the state of the object based on user input and inertia.
    ///
    /// This function:
    /// - Adjusts angular velocity when the Space or Enter keys are pressed.
    /// - Handles mouse drag behavior, updating the angle while dragging.
    /// - Computes release velocity when the mouse button is released.
    /// - Integrates inertia into the angle update.
    ///
    /// The update logic includes:
    /// - Increasing angular velocity when Space is held.
    /// - Resetting angular velocity when Enter is pressed.
    /// - Capturing the angle when the Left Mouse Button is dragged.
    /// - Calculating angular velocity based on the shortest angular distance upon release.
    /// - Applying clamped inertia to maintain smooth rotation.
    ///
    /// # Parameters
    /// - `self`: A mutable reference to the struct containing angle state.
    ///
    /// # Behavior
    /// - If the user presses Space, the angular velocity increases.
    /// - If the user presses Enter, the angular velocity resets.
    /// - If the user clicks and drags the mouse, the object follows the cursor.
    /// - If the user releases the mouse, angular velocity is computed based on the last dragging movement.
    /// - Regardless of input, inertia is applied to the angle to create smooth motion.
    ///
    /// This function should be called every frame to maintain interactive behavior.
    fn update(&mut self) {
        let center = vec2(screen_width() / 2., screen_height() / 2.);
        let delta = get_frame_time();
        let a_limit = 6.;

        // -- Handle Keyboard Functionality --
        // - Add constant acceleration
        // - Reset angular velocity
        if is_key_down(KeyCode::Space) {
            self.angle_v += 0.05;
        }

        if is_key_pressed(KeyCode::Enter) {
            self.angle_v = 0.;
        }

        // -- Handle Mouse Functionality --
        // - Click and Drag
        if is_mouse_button_down(MouseButton::Left) {
            let m_pos: Vec2 = mouse_position().into();
            let current_m_angle = (m_pos - center).to_angle();

            if !self.is_dragging {
                self.prev_m_angle = current_m_angle;
                self.is_dragging = true;
                self.angle_v = 0.; // no inertia while dragging.
            }

            // Snap baton to current mouse angle;
            self.angle = current_m_angle;

            // Update prev_m_angle so "previous" is one frame older.
            self.prev_m_angle = current_m_angle;

            // Since mouse is still down, don't attempt release logic.
            return;
        }

        if self.is_dragging && is_mouse_button_released(MouseButton::Left) {
            let m_pos: Vec2 = mouse_position().into();
            let current_m_angle = (m_pos - center).to_angle();

            // "How far we turned in the last frame of dragging."
            let raw_dtheta = shortest_angular_distance(self.prev_m_angle, current_m_angle);
            let dt_frame = get_frame_time();
            self.angle_v = raw_dtheta / dt_frame;

            self.is_dragging = false;
        }

        // apply linear angular drag
        let damping = 0.5;
        let alpha_drag = -damping * self.angle_v;
        self.angle_v += alpha_drag * delta;

        // If we reach here, either:
        //    • We were never dragging, or
        //    • We just computed a release velocity (angle_v), or
        //    • We’re idle (mouse-up and is_dragging==false).
        //
        // Integrate inertia in all those cases:
        self.angle_v = self.angle_v.clamp(-TAU * a_limit, TAU * a_limit);
        self.angle += self.angle_v * delta;
    }

    /// Draws the baton.
    fn draw(&mut self) {
        draw_rectangle_ex(
            self.x,
            self.y,
            self.w,
            self.h,
            DrawRectangleParams {
                offset: vec2(0.5, 0.5),
                rotation: self.angle,
                color: DARKPURPLE,
            },
        )
    }
}

/// Computes the shortest angular distance between two angles.
///
/// This function calculates the smallest difference between two angles `a` and `b`,
/// ensuring the result falls within the range `[-PI, PI]`. This is useful for determining
/// the shortest rotation direction in circular motion.
///
/// # Parameters
/// - `a`: The initial angle in radians.
/// - `b`: The target angle in radians.
///
/// # Returns
/// - A `f32` value representing the shortest angular difference in radians.
///
/// # Behavior
/// - Computes the angular difference while wrapping within the `TAU` range.
/// - Adjusts the difference to ensure it falls within `[-PI, PI]`, preventing
///   unnecessary long rotations.
fn shortest_angular_distance(a: f32, b: f32) -> f32 {
    let mut diff = (b - a) % TAU;
    if diff > PI {
        diff -= TAU;
    } else if diff < -PI {
        diff += TAU;
    }
    diff
}
