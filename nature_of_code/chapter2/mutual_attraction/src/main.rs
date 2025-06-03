use macroquad::prelude::*;

#[macroquad::main("Mutual Attraction")]
async fn main() {
    let mut movers = vec![];

    for _ in 0..10 {
        let r_x = rand::gen_range(100., 200.);
        let r_y = rand::gen_range(100., 200.);
        let r_m = rand::gen_range(10., 25.);
        let mover = Mover::new(
            vec2(screen_width() / 2. - r_x, screen_height() / 2. - r_y),
            0. + r_m,
        );
        movers.push(mover);
    }

    let mut attractor = Attractor::new(vec2(screen_width() / 2., screen_height() / 2.), 75.);

    loop {
        clear_background(BLACK);

        for mover in &mut movers {
            attractor.attract(mover);
            mover.update();
            mover.show();
        }
        attractor.show();

        if is_mouse_button_down(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            let mouse_pos = vec2(mouse_x, mouse_y);
            attractor.pos = mouse_pos;
        }
        
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}

struct Attractor {
    pos: Vec2,
    mass: f32,
    r: f32,
    color: Color,
}

impl Attractor {
    fn new(pos: Vec2, mass: f32) -> Self {
        let r = f32::sqrt(mass) * 2.;
        let color = Color::from_rgba(100, 100, 100, 255);
        Self {
            pos,
            mass,
            r,
            color,
        }
    }

    fn show(&self) {
        draw_circle(self.pos.x, self.pos.y, self.r, self.color);
    }

    fn attract(&self, mover: &mut Mover) {
        let mut force = self.pos - mover.pos;
        let distance_sq = force.length_squared().clamp(10., 1000.);
        let g = 2.;
        let str: f32 = g * (self.mass * mover.mass) / distance_sq;

        set_mag(&mut force, str);

        mover.apply_force(force);
    }
}

struct Mover {
    pos: Vec2,
    vel: Vec2,
    acc: Vec2,
    r: f32,
    mass: f32,
    color: Color,
}

impl Mover {
    fn new(pos: Vec2, mass: f32) -> Self {
        let r_x = rand::gen_range(10., 20.);
        let r_y = rand::gen_range(-10., -20.);
        let vel = Vec2::new(r_x, r_y);
        let acc = Vec2::splat(0.);
        let r = f32::sqrt(mass) * 2.;
        let color = Color::from_rgba(200, 200, 200, 150);

        Self {
            pos,
            vel,
            acc,
            r,
            mass,
            color,
        }
    }

    fn show(&self) {
        draw_circle(self.pos.x, self.pos.y, self.r, self.color);
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
}

// Helpers
fn set_mag(vec: &mut Vec2, scalar: f32) -> Vec2 {
    *vec = vec.normalize();
    vec.x *= scalar;
    vec.y *= scalar;
    *vec
}
