#![allow(unused)]

use macroquad::prelude::*;

#[macroquad::main("Exercise 1.5")]
async fn main() {

    let mut mover = Mover::new(
        Vec2::new(
            screen_width() / 2.,
            screen_height() / 2.)
    );

    loop{
        clear_background(BLACK);

        mover.show();
        mover.thrust();
        mover.wrap_screen();

        //dbg!(mover.vel);

        next_frame().await;
    }
}

struct Mover {
    pos: Vec2,
    vel: Vec2,
    acc: Vec2,
    height: f32,
    base: f32,
    rot: f32,
}

impl Mover {
    fn new(pos: Vec2) -> Mover {
        Mover {
            pos,
            vel: Vec2::splat(0.),
            acc: Vec2::splat(0.),
            height: 25.,
            base: 22.,
            rot: 0.,
        }
    }

    fn thrust(&mut self) {
        let delta_time = get_frame_time();
        if is_key_down(KeyCode::Up) {
            self.acc = Vec2::new(self.rot.sin(), -self.rot.cos()) / 3.;
        } else {
            self.acc = Vec2::splat(0.);
        }
        if is_key_down(KeyCode::Right) {
            self.rot += 5. * delta_time;
        } else if is_key_down(KeyCode::Left) {
            self.rot -= 5. * delta_time;
        }
        self.vel += self.acc;
        if self.vel.length() > 5. {
            self.vel = self.vel.normalize() * 5.;
        }
        if is_key_down(KeyCode::Down) {
            self.vel = apply_friction(self.vel.x, self.vel.y);
        }
        self.pos += self.vel;
    }

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

fn apply_friction(x_vel: f32, y_vel: f32) -> Vec2 {
    let x = x_vel * 0.95;
    let y = y_vel * 0.95;
    Vec2::new(x, y)
}
