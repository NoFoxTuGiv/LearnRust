use macroquad::prelude::*;
use miniquad::date;

#[macroquad::main("3D Bounce")]
async fn main() {
    const CENTER: Vec3 = Vec3::new(0., 0., 0.);
    const WIDTH: f32 = 30.;
    const ROTATION_SPEED: f32 = 0.5;

    //Ball w/ random velocity vector
    rand::srand(date::now() as u64);
    let rx: f32 = rand::gen_range(-0.5, 0.5);
    let ry: f32 = rand::gen_range(-0.5, 0.5);
    let rz: f32 = rand::gen_range(-0.5, 0.5);
    let mut ball = Ball::new(CENTER, Vec3::new(rx, ry, rz));

    let mut c_angle: f32 = 0.;

    loop{
        clear_background(BLACK);


        //Camera Rotation
        let delta_time = get_frame_time();
        c_angle += delta_time * ROTATION_SPEED;

        let c_radius = 50.0;
        let c_height = 30.0;
        let cam_x = CENTER.x + c_radius * f32::cos(c_angle);
        let cam_z = CENTER.z + c_radius * f32::sin(c_angle);
        let cam_y = CENTER.x + c_height;

        set_camera(&Camera3D {
            position: vec3(cam_x, cam_y, cam_z),
            up: vec3(0., 1., 0.),
            target: CENTER,
            ..Default::default()
        });

        draw_cube_wires(CENTER, Vec3::splat(WIDTH), PURPLE);

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
        draw_sphere_wires(self.pos, self.r, None, self.color);
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
    }
}
