#![allow(unused)]

use macroquad::prelude::*;

pub struct Vehicle {
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    height: f32,
    base: f32,
    rotation: f32,
    max_speed: f32,
    max_force: f32,
    targets: Vec<Vec2>,
    threats: Vec<Vec2>,
}

impl Vehicle {
    // --- NUSA (New, Update, Show, Apply_Force) ---
    pub fn new() -> Vehicle {
        let position = Vec2::new(
            rand::gen_range(25., screen_height() - 25.),
            rand::gen_range(25., screen_width() - 25.),
        );
        let velocity = Vec2::new(rand::gen_range(-5., 5.), rand::gen_range(-5., 5.));
        let acceleration = Vec2::splat(0.);

        Vehicle {
            position,
            velocity,
            acceleration,
            height: 25.,
            base: 22.,
            rotation: 0.,
            max_speed: 4.,
            max_force: 0.1,
            targets: vec![],
            threats: vec![],
        }
    }

    pub fn update(&mut self) {
        self.rotation = self.velocity.y.atan2(self.velocity.x) + std::f32::consts::FRAC_PI_2;
        // self.wrap_screen(); // Eventually ditch this, assuming the vehicle will always prefer to return to the screen.

        self.apply_friction();
        self.velocity += self.acceleration;
        self.position += self.velocity;

        self.acceleration *= 0.;
    }

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

    pub fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force;
    }

    // --- Getters ---
    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    // --- Behaviors ---
    pub fn seek(&mut self, target: &Vec2) -> Vec2 {
        let desired = self.get_desired_velocity(target, true);
        (desired - self.velocity).clamp_length_max(self.max_force)
    }

    pub fn flee(&mut self, threat: &Vec2) -> Vec2 {
        let desired = -self.get_desired_velocity(threat, false);
        (desired - self.velocity).clamp_length_max(self.max_force)
    }

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
            return (desired - self.velocity).clamp_length_max(self.max_force);
        }

        Vec2::ZERO
    }

    // --- Private Fns ---
    fn get_desired_velocity(&mut self, target: &Vec2, arrive: bool) -> Vec2 {
        let mut desired: Vec2 = *target - self.position;
        let distance: f32 = desired.length();
        let speed: f32 = if arrive && distance < 100.0 {
            map_range(distance, 0., 100., 0., self.max_speed)
        } else {
            self.max_speed
        };
        desired.normalize_or_zero() * speed
    }

    fn apply_friction(&mut self) {
        self.velocity *= 0.999;
    }

    fn wrap_screen(&mut self) {
        match self.position.x {
            x if x <= 0. => self.position.x = screen_width(),
            x if x >= screen_width() => self.position.x = 0.,
            _ => {}
        }

        match self.position.y {
            y if y <= 0. => self.position.y = screen_height(),
            y if y >= screen_height() => self.position.y = 0.,
            _ => {}
        }
    }
}

// --- Helper Functions ---
fn map_range(value: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    out_min + (out_max - out_min) * ((value - in_min) / (in_max - in_min))
}
