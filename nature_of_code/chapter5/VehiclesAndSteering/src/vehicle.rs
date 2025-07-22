#![allow(unused)]

use std::f32::consts::{PI, TAU};

use macroquad::{prelude::*, rand::gen_range};

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
    /// List of target locations the vehicle may be attracted to
    targets: Vec<Vec2>,
    /// List of threat locationst the vehicle may be repulsed by
    threats: Vec<Vec2>,
    /// Flee radius for this vehicle
    pub flee_radius: f32,
    /// Wander direction
    wander_theta: f32,
    /// New wander direction cooldown
    wander_cd: f32,
    /// Toggles debug information for vehicle movement
    debug: bool,
}

impl Vehicle {
    const MAX_SPEED: f32 = 0.666;
    const MAX_FORCE: f32 = 0.003;
    const WANDER_CIRCLE_DIST: f32 = 100.;
    const WANDER_CIRCLE_RADIUS: f32 = 25.;
    const ARRIVAL_THRESHOLD: f32 = 100.;
    const FRICTION_FACTOR: f32 = 0.999;
    const FLEE_RADIUS: f32 = 250.;

    // --- NUSA (New, Update, Show, Apply_Force) ---

    /// Creates a new 'Vehicle' with a random position and initial velocity.
    ///
    /// The vehicle is initialized with default parameters for its size, speed, and force limits
    ///
    /// # Returns
    /// A new 'Vehicle' instance.
    pub fn new() -> Vehicle {
        let position = Vec2::new(
            rand::gen_range(25., screen_width() - 25.),
            rand::gen_range(25., screen_height() - 25.),
        );
        let velocity = Vec2::new(rand::gen_range(-3., 3.), rand::gen_range(-3., 3.));
        let acceleration = Vec2::splat(0.);
        let wander_theta = gen_range(0., TAU);

        Vehicle {
            position,
            velocity,
            acceleration,
            height: 25.,
            base: 22.,
            rotation: 0.,
            max_speed: Self::MAX_SPEED,
            max_force: Self::MAX_FORCE,
            targets: vec![],
            threats: vec![],
            flee_radius: Self::FLEE_RADIUS,
            wander_theta,
            wander_cd: 0.,
            debug: false,
        }
    }

    /// Updats the vehicle's state for the current frame.
    ///
    /// This method applies physics:
    /// 1. Rotates the vehicle to face its direction of travel.
    /// 2. Applies friction to the velocity.
    /// 3. Updates the velocity with acceleration created via `apply_force()`
    /// 4. Updates the position with velocity.
    /// 5. Resets acceleration for the next frame.
    pub fn update(&mut self) {
        self.rotation = self.velocity.y.atan2(self.velocity.x) + std::f32::consts::FRAC_PI_2;

        self.apply_friction();
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

    // --- Setters ---

    /// Toggles debug boolean for the vehicle.
    pub fn toggle_debug(&mut self) {
        self.debug = !self.debug;
    }

    // --- Behaviors ---

    /// Calculates a steering force to move towards a target position.
    ///
    /// This behavior includes an "arrival" component, causing the vehicle to slow
    /// as it approaches the target.
    ///
    /// This Vector should usually be applied via a call to `apply_force()`
    ///
    /// # Arguments
    /// * `target` - A reference to the `Vec2` target position.
    ///
    /// # Returns
    /// A `Vec2` steering force vector, clamped to `max_force`.
    pub fn seek(&mut self, target: &Vec2) -> Vec2 {
        let desired = self.get_desired_velocity(target, true, self.max_speed);
        (desired - self.velocity).clamp_length_max(self.max_force)
    }

    /// Calculates a steering force to move away from a threat position.
    ///
    /// This Vector should usually be applied via a call to `apply_force()`
    ///
    /// # Arguments
    /// * `threat` - A reference to the `Vec2` threat position.
    /// * `flee_radius` - The distance within which the vehicle will begin to flee
    ///   from the threat
    ///
    /// # Returns
    /// A `Vec2` steering force vector, clamped to `max_force`.
    pub fn flee(&mut self, threat: &Vec2, flee_radius: f32) -> Vec2 {
        let distance = self.position.distance(*threat);

        if distance < flee_radius {
            let desired = -self.get_desired_velocity(threat, false, self.max_speed);
            (desired - self.velocity).clamp_length_max(self.max_force)
        } else {
            Vec2::ZERO
        }
    }

    /// Implements a "wander" steering behavior.
    ///
    /// The vehicle attempts to move towards a point on a circle in front of it,
    /// with the target point on the circle changing randomly over time.
    pub fn wander(&mut self) {
        let vel = self.velocity.normalize_or_zero();
        let (cx, cy) = (
            vel.x * Self::WANDER_CIRCLE_DIST + self.position.x,
            vel.y * Self::WANDER_CIRCLE_DIST + self.position.y,
        );
        let r = Self::WANDER_CIRCLE_RADIUS;
        let theta = self.wander_theta;
        let x = cx + r * f32::cos(theta);
        let y = cy + r * f32::sin(theta);
        let wander_vec = Vec2::new(x, y);

        if self.wander_cd > 0.5 {
            self.wander_theta = gen_range(0., TAU);
            self.wander_cd = 0.;
        }

        let mut wander = self.get_desired_velocity(&wander_vec, false, self.max_speed * 0.50);
        wander = (wander - self.velocity).clamp_length_max(self.max_force * 0.66);

        self.apply_force(wander);

        self.wander_cd += get_frame_time();

        // Debug drawing
        if self.debug {
            draw_ellipse_lines(cx, cy, r, r, theta, 2., WHITE);
            draw_line(cx, cy, x, y, 2., WHITE);
        }
    }

    /// Calculates a steering force to keep the vehicle within the screen boundaries.
    ///
    /// The force increases proportionally to the vehicle's distance into the margin.
    ///
    /// This Vector should usually be applied via a call to `apply_force()`
    ///
    /// # Arguments
    /// * `margin` - The distance from the screen edge where the boundary force starts
    ///
    /// # Returns
    /// A `Vec2` steering force vector if the vehicle is within the margin, otherwise,
    /// `Vec2::ZERO`
    pub fn boundary_force(&mut self, margin: f32) -> Vec2 {
        let mut force = Vec2::ZERO;

        // Horizontal Boundaries
        if self.position.x < margin {
            let distance = margin - self.position.x;
            force.x = map_range(distance, 0., margin, 0., self.max_speed);
        } else if self.position.x > screen_width() - margin {
            let distance = self.position.x - (screen_width() - margin);
            force.x = -map_range(distance, 0., margin, 0., self.max_speed);
        }

        // Vertical boundaries
        if self.position.y < margin {
            let distance = margin - self.position.y;
            force.y = map_range(distance, 0.0, margin, 0.0, self.max_speed);
        } else if self.position.y > screen_height() - margin {
            let distance = self.position.y - (screen_height() - margin);
            force.y = -map_range(distance, 0.0, margin, 0.0, self.max_speed);
        }

        if force != Vec2::ZERO {
            let desired = force.normalize_or_zero() * self.max_speed;
            return (desired - self.velocity).clamp_length_max(self.max_force * 1.33);
        }

        Vec2::ZERO
    }

    // --- Private Fns ---

    /// A helper function to calculate the desired velocity towards a target.
    ///
    /// If `arrive` is true, the speed is scaled down as the vehicle approaches the target.
    ///
    /// # Arguments
    /// * `target` - The position to move towards
    /// * `arrive` - A boolean indicating whether to enable arrival behavior.
    ///
    /// # Returns
    /// A `Vec2` representing the desired velocity.
    fn get_desired_velocity(&mut self, target: &Vec2, arrive: bool, max_speed: f32) -> Vec2 {
        let mut desired: Vec2 = *target - self.position;
        let distance: f32 = desired.length();
        let speed: f32 = if arrive && distance < Self::ARRIVAL_THRESHOLD {
            map_range(distance, 0., 100., 0., self.max_speed)
        } else {
            max_speed
        };
        desired.normalize_or_zero() * speed
    }

    /// Applies a small amount of friction to the vehicle's velocity every frame.
    fn apply_friction(&mut self) {
        self.velocity *= Self::FRICTION_FACTOR;
    }
}

// --- Helper Functions ---

/// Remaps a number from one range to another.
///
/// # Arguments
/// * `value` - The incoming value to be converted.
/// * `in_min` - The lower bound of the value's current range.
/// * `in_max` - The upper bound of the value's current range.
/// * `out_min` - The lower bound of the value's target range.
/// * `out_max` - The upper bound of the value's target range.
///
/// # Type Parameters
/// * `T` - A type that supports basic arithmetic (+, -, *, /) and can be copied.
///   (i.e. `f32`, `f64`, `i32`, `u64`, etc., etc.)
///
/// # Returns
/// The remapped value as an `f32`.
fn map_range<T>(value: T, in_min: T, in_max: T, out_min: T, out_max: T) -> T
where
    T: std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + Copy,
{
    out_min + (out_max - out_min) * ((value - in_min) / (in_max - in_min))
}
