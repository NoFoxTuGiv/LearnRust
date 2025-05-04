#![allow(unused)]

use macroquad::prelude::*;

#[macroquad::main("3D Bounce")]
async fn main() {
    const CENTER: Vec3 = Vec3::new(0., 0., 0.);

    let mut ball = Ball::new(CENTER, Vec3::splat(0.01));

    loop{
        clear_background(LIGHTGRAY);

        set_camera(&Camera3D {
            position: vec3(-25., 15., 20.0),
            up: vec3(0., 1., 0.),
            target: CENTER,
            ..Default::default()
        });

        draw_cube_wires(CENTER, vec3(15., 15., 15.), PURPLE);

        ball.update();
        ball.show();

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
        let color = BLACK;

        Ball { pos, vel, r, color }
    }
    fn update(&mut self) {
        self.pos += self.vel;
    }

    fn show(&self) {
        draw_sphere(self.pos, self.r, None, self.color);
    }
}
