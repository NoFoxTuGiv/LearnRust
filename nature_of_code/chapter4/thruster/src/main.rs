#![allow(unused)]

mod emitter;

use emitter::Emitter;
use macroquad::prelude::*;

#[macroquad::main(window_conf())]
async fn main() {
    let mut mover = Mover::new(Vec2::new(screen_width() / 2., screen_height() / 2.));

    loop {
        clear_background(BLACK);

        mover.show();
        mover.thrust();
        mover.wrap_screen();

        mover.emitter.run();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}

/// Represents a simple controllable physics body that can move,
/// rotate, accelerate, and wrap around the screen edges.
struct Mover {
    /// Position of the mover on screen.
    pos: Vec2,
    /// Current velocity vector.
    vel: Vec2,
    /// Current acceleration vector (resets every frame).
    acc: Vec2,
    /// Visual height of the triangle shape.
    height: f32,
    /// Visual base width of the triangle shape.
    base: f32,
    /// Current rotation angle in radians.
    rot: f32,
    /// Particle Emitter.
    emitter: Emitter,
    /// Max Speed.
    max_speed: f32,
}

impl Mover {
    /// Creates a new mover at the specified position
    fn new(pos: Vec2) -> Mover {
        Mover {
            pos,
            vel: Vec2::splat(0.),
            acc: Vec2::splat(0.),
            height: 25.,
            base: 22.,
            rot: 0.,
            emitter: Emitter::new(pos.x, pos.y),
            max_speed: 2.,
        }
    }

    /// Applies control-based forces and updates position and velocity.
    ///
    /// - Arrow Up: thrust forward
    /// - Arrow Left/Right: rotate
    /// - Arrow Down: apply brakes
    fn thrust(&mut self) {
        let delta_time = get_frame_time();
        if is_key_down(KeyCode::Up) {
            self.acc = Vec2::new(self.rot.sin(), -self.rot.cos()) / 36.;
            // == Emitter ==
            self.emitter.update_origin(self.pos, self.rot);
            self.emitter.add_particle(self.rot);
        } else {
            self.acc = Vec2::splat(0.);
        }
        if is_key_down(KeyCode::Right) {
            self.rot += 5. * delta_time;
        } else if is_key_down(KeyCode::Left) {
            self.rot -= 5. * delta_time;
        }
        if is_key_down(KeyCode::Down) {
            self.vel *= 0.98;
        }

        // == Handle Physics ==
        self.vel += self.acc;
        // = Limit max speed =
        if self.vel.length() > self.max_speed {
            self.vel = self.vel.normalize() * self.max_speed;
        }
        self.pos += self.vel;
        self.vel = apply_friction(self.vel.x, self.vel.y);
    }

    /// Renders the mover as a triangle based on its position and rotation.
    fn show(&self) {
        let v1 = Vec2::new(
            self.pos.x + self.rot.sin() * self.height / 2.,
            self.pos.y - self.rot.cos() * self.height / 2.,
        );
        let v2 = Vec2::new(
            self.pos.x - self.rot.cos() * self.base / 2. - self.rot.sin() * self.height / 2.,
            self.pos.y - self.rot.sin() * self.base / 2. + self.rot.cos() * self.height / 2.,
        );
        let v3 = Vec2::new(
            self.pos.x + self.rot.cos() * self.base / 2. - self.rot.sin() * self.height / 2.,
            self.pos.y + self.rot.sin() * self.base / 2. + self.rot.cos() * self.height / 2.,
        );

        draw_triangle_lines(v1, v2, v3, 2., WHITE);
    }

    /// Wraps the mover around screen edges to create a toroidal space.
    fn wrap_screen(&mut self) {
        if self.pos.x > screen_width() {
            self.pos.x = 0.;
        }
        if self.pos.x < 0. {
            self.pos.x = screen_width();
        }
        if self.pos.y > screen_height() {
            self.pos.y = 0.;
        }
        if self.pos.y < 0. {
            self.pos.y = screen_height();
        }
    }
}

/// Applies friction to a 2D velocity vector by reducing each component.
///
/// Returns a dampened `Vec2` velocity.
fn apply_friction(x_vel: f32, y_vel: f32) -> Vec2 {
    let x = x_vel * 0.999;
    let y = y_vel * 0.999;
    Vec2::new(x, y)
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Thruster by NoFoxTuGiv".into(),
        ..Default::default()
    }
}
