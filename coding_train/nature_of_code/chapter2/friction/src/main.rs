use std::f32::consts::PI;

use macroquad::prelude::*;

const GRAVITY: Vec2 = vec2(0., 1.);
const WIND: Vec2 = vec2(1.6, 0.);

#[macroquad::main("Friction")]
async fn main() {
    
    let mut movers: Vec<Mover> = Vec::new();

    for i in 0..5 {
        let mover = Mover::new(
            vec2(
                screen_width() / 6. + (i as f32 * 150.),
                screen_height() / 3., 
            ),
            40. - i as f32 * 5.,
        );
        movers.push(mover);
    }

    loop {
        clear_background(BLACK);

        if is_key_down(KeyCode::Space) {
            for mover in &mut movers {
                mover.apply_force(WIND);
            }
        }

        if is_mouse_button_down(MouseButton::Left) {
            let mouse_pos: Vec2 = mouse_position().into();
            for mover in &mut movers {
                if mover.mouse_over(mouse_pos) {
                    mover.is_clicked = true;
                }
                if mover.is_clicked {
                    mover.pos = mouse_position().into();
                    mover.vel = vec2(0., 0.);
                }
            }
        }
        if is_mouse_button_released(MouseButton::Left) {
            for mover in &mut movers {
                if mover.is_clicked {
                    mover.acc += mouse_delta_position() * -1000.;
                }
                mover.is_clicked = false;
            }
        }

        for mover in &mut movers {
            if is_mouse_button_pressed(MouseButton::Right) {
                mover.pos = mover.start_pos;
            }
            if mover.contacting_edge() {
                let c = 0.05;
                let mut friction = mover.vel;
                friction *= vec2(-1., -1.);
                friction = set_mag(&mut friction, c);

                mover.apply_force(friction);
            }

            mover.apply_force(GRAVITY * mover.mass);
            mover.edges();
            mover.update();
            mover.show();
        }

        next_frame().await;
    }
}

fn set_mag(v: &mut Vec2, m: f32) -> Vec2 {
    let mut new_v = v.normalize();
    new_v.x *= m;
    new_v.y *= m;
    new_v
}

struct Mover {
    pos: Vec2,
    start_pos: Vec2,
    vel: Vec2,
    acc: Vec2,
    r: f32,
    mass: f32,
    is_clicked: bool,
}

impl Mover {
    fn new(pos: Vec2, r: f32) -> Self {
        let start_pos = pos;
        let vel = Vec2::splat(0.);
        let acc = Vec2::splat(0.);
        let mass = PI * r * r / 500.;
        let is_clicked = false;

        Self { pos, start_pos, vel, acc, r, mass, is_clicked }
    }

    fn show(&self) {
        draw_circle(self.pos.x, self.pos.y, self.r, DARKPURPLE);
    }

    fn update(&mut self) {
        let d_time = get_frame_time();
        let speed = 10.;

        self.vel += self.acc;
        self.pos += self.vel * d_time * speed;
        self.acc = vec2(0., 0.);
    }

    fn apply_force(&mut self, force: Vec2) {
        let f = force / self.mass;
        self.acc += f;
    }

    fn edges(&mut self) {
        let bounce_scalar = -0.9;

        if self.pos.y > screen_height() - self.r {
            self.pos.y = screen_height() - self.r;
            self.vel.y *= bounce_scalar;
        }
        if self.pos.x > screen_width() - self.r {
            self.pos.x = screen_width() - self.r;
            self.vel.x *= bounce_scalar;
        } else if self.pos.x < self.r {
            self.pos.x = self.r;
            self.vel.x *= bounce_scalar;
        }
    }

    // fn is_colliding(&mut self, mover: &mut Mover) -> bool {
    //     ((self.pos + self.r) + (mover.pos + mover.r)).length() <= 0.
    // } 

    fn contacting_edge(&self) -> bool {
        self.pos.y > screen_height() - self.r - 1.
    }

    fn mouse_over(&self, mouse_pos: Vec2) -> bool {
        mouse_pos.x > self.pos.x - self.r &&
        mouse_pos.x < self.pos.x + self.r &&
        mouse_pos.y > self.pos.y - self.r &&
        mouse_pos.y < self.pos.y + self.r
    }
}
