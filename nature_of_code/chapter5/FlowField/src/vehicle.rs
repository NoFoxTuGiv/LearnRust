use macroquad::{ prelude::*, rand::gen_range };
use crate::FlowField;

/// Represents an autonomous agent (a "vehicle") that can navigate its environment
/// using steering behaviors.
pub struct Vehicle {
    /// Current location
    pub position: Vec2,
    /// Speed and direction.
    velocity: Vec2,
    /// Current rate of change in velocity. Reset every frame via update()
    acceleration: Vec2,
    /// Height of the triangle used to draw the vehicle.
    height: f32,
    /// Width of the base of the triangle
    base: f32,
    /// Current orientation in radians.
    rotation: f32,
    /// Max magnitude of the velocity vector
    max_speed: f32,
    /// Max magnitude of any steering force applied
    max_force: f32,
}

impl Vehicle {
    const MAX_SPEED: f32 = 1.0;
    const MAX_FORCE: f32 = 0.006;

    // --- NUSA (New, Update, Show, Apply_Force) ---

    /// Creates a new 'Vehicle' with a random position and initial velocity.
    ///
    /// The vehicle is initialized with default parameters for its size, speed, and force limits
    ///
    /// # Returns
    /// A new 'Vehicle' instance.
    pub fn new() -> Vehicle {
        let position = Vec2::new(
            gen_range(25., screen_width() - 25.),
            gen_range(25., screen_height() - 25.),
        );
        let velocity = Vec2::new(gen_range(-3., 3.), gen_range(-3., 3.));
        let acceleration = Vec2::splat(0.);

        Vehicle {
            position,
            velocity,
            acceleration,
            height: 15.,
            base: 10.,
            rotation: 0.,
            max_speed: Self::MAX_SPEED,
            max_force: Self::MAX_FORCE,
        }
    }

    /// Updates the vehicle's position and orientation for the current frame.
    ///
    /// This method handles the basic physics:
    /// 1. Wraps the vehicle around the screen borders if it goes off-screen.
    /// 2. Rotates the vehicle to point in the direction it's moving.
    /// 3. Applies the accumulated acceleration to update the velocity.
    /// 4. Clamps the velocity to ensure it doesn't exceed `max_speed`.
    /// 5. Updates the vehicle's position based on its velocity.
    /// 6. Resets the acceleration to zero, ready for new forces in the next frame.
    pub fn update(&mut self) {
        self.borders();
        self.rotation = self.velocity.y.atan2(self.velocity.x) + std::f32::consts::FRAC_PI_2;

        self.velocity += self.acceleration;
        self.velocity = self.velocity.clamp_length_max(self.max_speed);
        self.position += self.velocity;

        self.acceleration = Vec2::ZERO;
    }

    /// Renders the vehicle as a triangle on the screen.
    pub fn show(&self) {
        let v1 = Vec2::new(
            self.position.x + self.rotation.sin() * self.height / 2.,
            self.position.y - self.rotation.cos() * self.height / 2.,
        );
        let v2 = Vec2::new(
            self.position.x
                - self.rotation.cos() * self.base / 2.
                - self.rotation.sin() * self.height / 2.,
            self.position.y - self.rotation.sin() * self.base / 2.
                + self.rotation.cos() * self.height / 2.,
        );
        let v3 = Vec2::new(
            self.position.x + self.rotation.cos() * self.base / 2.
                - self.rotation.sin() * self.height / 2.,
            self.position.y
                + self.rotation.sin() * self.base / 2.
                + self.rotation.cos() * self.height / 2.,
        );

        draw_triangle_lines(v1, v2, v3, 2., WHITE);
    }

    /// Adds force vector to vehicle's acceleration.
    ///
    /// Forces accumulate over a frame and are applied during the `update()` call.
    ///
    /// # Arguments
    /// * `force` - The `Vec2` force to apply.
    pub fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force.clamp_length_max(self.max_force);
    }

    /// Steers the vehicle to follow the direction indicated by the `FlowField` at its current position.
    ///
    /// This method calculates a 'desired' velocity based on the flow field and then
    /// computes a 'steering' force to align the vehicle's current velocity with this desired velocity.
    ///
    /// # Arguments
    /// * `flow` - A reference to the `FlowField` that the vehicle should follow.
    pub fn follow(&mut self, flow: &FlowField) {
        let mut desired = flow.lookup(&self.position);
        desired *= self.max_speed;

        let mut steer = desired - self.velocity;
        steer = steer.clamp_length_max(self.max_force);
        self.apply_force(steer);
    }

    /// Handles screen wrapping for the vehicle.
    ///
    /// If the vehicle moves off one edge of the screen, it reappears on the opposite edge.
    fn borders(&mut self) {
        match self.position.x {
            x if x <= 0. => self.position.x = screen_width(), // Left to Right
            x if x >= screen_width() => self.position.x = 0., // Right to Left
            _ => {}
        }

        match self.position.y {
            y if y <= 0. => self.position.y = screen_height(), // Top to Bottom
            y if y >= screen_height() => self.position.y = 0., // Bottom to Top
            _ => {}
        }
    }
}
