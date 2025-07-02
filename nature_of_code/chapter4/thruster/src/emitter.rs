#![allow(unused)]

use macroquad::prelude::*;

/// A single particle with position, velocity, acceleration, and lifespan.
pub struct Particle {
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    lifespan: f32,
}

impl Particle {
    /// Creates a new `Particle` at the given position `(x, y)` with an initial directional velocity.
    ///
    /// # Parameters
    /// - `x`, `y`: Initial position of the particle.
    /// - `dir`: Direction angle (in radians) from which the particle is emitted.
    ///
    /// # Returns
    /// A newly initialized `Particle`.
    pub fn new(x: f32, y: f32, dir: f32) -> Particle {
        let position = Vec2::new(x, y);
        let direction = Vec2::new(dir.sin(), -dir.cos());
        let velocity = direction * -2. + Vec2::new(rand::gen_range(-0.5, 0.5), rand::gen_range(-0.5, 0.5));

        Particle {
            position,
            velocity,
            acceleration: Vec2::splat(0.),
            lifespan: 0.5, // Lifespan in seconds
        }
    }

    /// Updates the particle's physics state and decreases its lifespan.
    fn update(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity * get_frame_time() * 10.;
        self.lifespan -= get_frame_time(); // Reduce lifespan by elapsed time every frame.
        self.acceleration *= 0.;
    }

    /// Draws the particle utilizing a fade to red effect.
    fn show(&self) {
        let t = 1. - (self.lifespan / 0.5);
        let r = 1.;
        let g = 0.9 * (1. - t);     // Fades from 1.0 to 0.0
        let b = 0.2 * (1.0 - t);   // Fades from 0.8 to 0.0
        let a = self.lifespan * 2.;
        draw_circle(
            self.position.x,
            self.position.y,
            5.,
            Color::new(r, g, b, a)
        );
    }

    /// Applies an external force to the particle's acceleration.
    ///
    /// # Parameters
    /// - `force`: The vector force to be added to the acceleration.
    fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force;
    }

    /// Checks if the particle's lifespan has expired.
    ///
    /// # Returns
    /// `true` if the particle is considered dead; otherwise, `false`.
    pub fn is_dead(&self) -> bool {
        self.lifespan < 0.
    }

    /// Updates and renders the particle for the current frame.
    pub fn run(&mut self) {
        self.update();
        self.show();
    }
}

/// An emitter that spawns and manages a collection of particles at a given origin.
pub struct Emitter {
    particles: Vec<Particle>,
    origin: Vec2,
    rotation: f32,
}

impl Emitter {
    /// Creates a new `Emitter` at the specified position `(x, y)`.
    ///
    /// # Returns
    /// A new emitter with no initial particles.
    pub fn new(x: f32, y: f32) -> Emitter {
        Emitter {
            particles: vec![],
            origin: vec2(x, y),
            rotation: 0.,
        }
    }

    /// Emits a new particle in the specified direction.
    ///
    /// # Parameters
    /// - `direction`: The angle in radians used to compute the initial velocity of the particle.
    pub fn add_particle(&mut self, direction: f32) {
        let exhaust_offset = Vec2::new(self.rotation.sin(), -self.rotation.cos()) * -10.;
        let spawn = self.origin + exhaust_offset;

        self.particles
            .push(Particle::new(spawn.x, spawn.y, direction));
    }

    /// Updates the emitter's position and rotation.
    ///
    /// # Parameters
    /// - `new_origin`: The new origin vector for the emitter.
    /// - `new_rot`: The new rotation value in radians.
    pub fn update_origin(&mut self, new_origin: Vec2, new_rot: f32) {
        self.origin = new_origin;
        self.rotation = new_rot;
    }

    /// Updates and renders all particles, removing those that have expired.
    pub fn run(&mut self) {
        for particle in &mut self.particles {
            particle.run();
        }

        self.particles.retain(|particle| !particle.is_dead());
    }
}
