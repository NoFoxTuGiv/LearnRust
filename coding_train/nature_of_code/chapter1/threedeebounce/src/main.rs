//TODO:
//
// - Rotate camera over time.
#![allow(unused)]

use macroquad::prelude::*;

#[macroquad::main("3D Bounce")]
async fn main() {
    const CENTER: Vec3 = Vec3::new(0., 0., 0.);
    const WIDTH: f32 = 15.;

    let mut ball = Ball::new(CENTER, Vec3::new(0.4, 0.2, 0.3));

    loop{
        clear_background(LIGHTGRAY);

        set_camera(&Camera3D {
            position: vec3(-25., 15., 20.0),
            up: vec3(0., 1., 0.),
            target: CENTER,
            ..Default::default()
        });

        draw_cube_wires(CENTER, Vec3::splat(WIDTH), PURPLE);
        // draw_cube(CENTER, Vec3::splat(WIDTH), None, PURPLE);

        ball.update();
        ball.show();
        ball.edges(WIDTH);

        set_default_camera();

        next_frame().await;
    }
}

struct Ball {
    pos: Vec3,
    vel: Vec3,
    r: f32,
    color: Color,
}

impl Ball {
    fn new( pos: Vec3, vel: Vec3) -> Ball {
        let r: f32 = 1.;
        let color = BLUE;

        Ball { pos, vel, r, color }
    }
    fn update(&mut self) {
        self.pos += self.vel;
    }

    fn show(&self) {
        draw_sphere(self.pos, self.r, None, self.color);
    }

    fn edges(&mut self, width: f32) {
        let pos_wall = (width / 2.) - self.r;
        let neg_wall = (-width / 2.) + self.r;
        if self.pos.x > pos_wall || self.pos.x < neg_wall {
            self.vel.x *= -1.;
        }
        if self.pos.y > pos_wall || self.pos.y < neg_wall {
            self.vel.y *= -1.;
        }
        if self.pos.z > pos_wall || self.pos.z < neg_wall {
            self.vel.z *= -1.;
        }
        // dbg!(self.pos);
    }
}
