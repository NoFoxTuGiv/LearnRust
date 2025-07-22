use macroquad::prelude::*;

/// Represents a single particle in a particle system.
pub struct Particle {
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    lifespan: f32,
}

impl Particle {
    /// Creates a new `Particle` at the specified `(x, y)` coordinates.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the particle's starting position.
    /// * `y` - The y-coordinate of the particle's starting position.
    pub fn new(x: f32, y: f32) -> Particle {
        let position = Vec2::new(x, y);
        let velocity = Vec2::new(rand::gen_range(-2., 2.), rand::gen_range(-4., 0.));
        Particle {
            position,
            velocity,
            acceleration: Vec2::splat(0.),
            lifespan: 1.0,
        }
    }

    /// Updates the particle's position, velocity, and lifespan.
    ///
    /// This method should be called every frame to simulate motion.
    fn update(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity * get_frame_time() * 10.0;
        self.lifespan -= get_frame_time();
        self.acceleration *= 0.0;
    }

    /// Draws the particle to the screen as a small circle.
    ///
    /// The alpha value of the circle fades with the particle's lifespan.
    fn show(&self) {
        draw_circle(
            self.position.x,
            self.position.y,
            5.0,
            Color::new(120. / 255., 81. / 255., 169. / 255., self.lifespan),
        );
    }

    /// Applies a force (acceleration) to the particle.
    ///
    /// # Arguments
    ///
    /// * `force` - A `Vec2` representing the directional force applied.
    fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force;
    }

    /// Checks if the particle's lifespan has expired.
    ///
    /// # Returns
    ///
    /// `true` if the lifespan is less than zero, indicating the particle is dead.
    pub fn is_dead(&self) -> bool {
        self.lifespan < 0.0
    }

    /// Applies gravity and updates the particle's state.
    ///
    /// This is a convenience method that encapsulates the full update cycle.
    pub fn run(&mut self) {
        let updraft = Vec2::new(0.0, -0.05);
        self.apply_force(updraft);
        self.update();
        self.show();
    }
}

/// A particle emitter that manages a collection of particles.
pub struct Emitter {
    particles: Vec<Particle>,
    origin: Vec2,
}

impl Emitter {
    /// Creates a new `Emitter` centered on the screen.
    pub fn new() -> Emitter {
        Emitter {
            particles: vec![],
            origin: vec2(screen_width() / 2.0, screen_height() / 2.0),
        }
    }

    /// Adds a new particle at the emitter's origin.
    pub fn add_particle(&mut self) {
        self.particles
            .push(Particle::new(self.origin.x, self.origin.y));
    }

    /// Updates the emitter's origin to a new position.
    ///
    /// # Arguments
    ///
    /// * `new_pos` - A `Vec2` representing the new origin position.
    #[allow(unused)]
    pub fn update_origin(&mut self, new_pos: Vec2) {
        self.origin = new_pos;
    }

    /// Updates and draws all active particles, removing any that are dead.
    pub fn run(&mut self) {
        for particle in &mut self.particles {
            particle.run();
        }

        self.particles.retain(|particle| !particle.is_dead());
    }
}

