use macroquad::prelude::*;
use miniquad::date;
use noise::{Fbm, MultiFractal, NoiseFn, Perlin};

const HELIUM: Vec2 = vec2(0., -1.);

#[macroquad::main("Exercise 2.1 - Balloon")]
async fn main() {
    rand::srand(date::now() as u64);

    let r = rand::gen_range(0, u32::MAX);
    let noise = Fbm::<Perlin>::new(r).set_octaves(2);

    let mut t = 0.;

    let mut balloon = Balloon {
        pos: vec2(screen_width() / 2., screen_height() / 2.),
        vel: Vec2::splat(0.),
        acc: Vec2::splat(0.),
        r: 40.,
    };

    loop {
        clear_background(BLACK);

        let wind = gen_wind(&noise, t);
        let wind_str = format!("Wind: {}", wind.x);

        draw_wind(&wind);
        draw_text(&wind_str, 25., screen_height() - 25., 20., WHITE);

        balloon.apply_force(wind);
        balloon.edges();
        balloon.update();
        balloon.show();

        t += 0.005;
        next_frame().await;
    }
}

fn gen_wind(noise: &Fbm<Perlin>, t: f64) -> Vec2 {
    let mut x = noise.get([t, 0.]);
    x *= 3.;
    vec2(x as f32, 0.)
}

fn draw_wind(wind: &Vec2) {
    let max_len = 50.;
    let start = vec2(screen_width() / 2., screen_height() - 25.);
    let end = vec2(start.x + (max_len * wind.x), screen_height() - 25.);
    draw_arrow(start, end, 2., WHITE);
}

fn draw_arrow(start: Vec2, end: Vec2, thick: f32, color: Color) {
    draw_line(start.x, start.y, end.x, end.y, thick, color);

    let dir = (start - end).normalize();

    let arrow_len = 10.;
    let angle = 25_f32.to_radians();

    let left = vec2(
        dir.x * angle.cos() - dir.y * angle.sin(),
        dir.x * angle.sin() + dir.y * angle.cos(),
    );

    let right = vec2(
        dir.x * angle.cos() + dir.y * angle.sin(),
        -dir.x * angle.sin() + dir.y * angle.cos(),
    );

    let left_tip = end + left * arrow_len;
    let right_tip = end + right * arrow_len;

    draw_line(end.x, end.y, left_tip.x, left_tip.y, thick, color);
    draw_line(end.x, end.y, right_tip.x, right_tip.y, thick, color);
}

struct Balloon {
    pos: Vec2,
    vel: Vec2,
    acc: Vec2,
    r: f32,
}

impl Balloon {
    fn apply_force(&mut self, force: Vec2) {
        let dtime = get_frame_time();
        self.acc += force * dtime;
    }

    fn edges(&mut self) {
        let top_edge_force = vec2(0., 20.);
        let left_edge_force = vec2(10., 0.);
        let right_edge_force = vec2(-10., 0.);
        if self.pos.y - self.r < 0. {
            self.apply_force(top_edge_force);
        }
        if self.pos.x - self.r < 0. {
            self.apply_force(left_edge_force);
        }
        if self.pos.x + self.r > screen_width() {
            self.apply_force(right_edge_force);
        }
    }

    fn update(&mut self) {
        self.apply_force(HELIUM);

        self.vel += self.acc;
        self.pos += self.vel;
        self.acc = vec2(0., 0.);
    }

    fn show(&self) {
        let num_seg = 20;
        let seg_len = 5.;
        let wave_amp = 7.;
        let wave_frq = 3.;
        let wave_spd = 4.;
        let time = get_time();

        for i in 0..num_seg {
            let y1 = (self.pos.y + 10.) + i as f32 * seg_len;
            let y2 = (self.pos.y + 10.) + (i + 1) as f32 * seg_len;

            let phase1 = (i as f32 * wave_frq + time as f32) * wave_spd;
            let phase2 = ((i + 1) as f32 * wave_frq + time as f32) * wave_spd;

            let x1 = self.pos.x + wave_amp * phase1.sin();
            let x2 = self.pos.x + wave_amp * phase2.sin();

            draw_line(x1, y1, x2, y2, 1.0, DARKGRAY);
        }

        draw_circle(self.pos.x, self.pos.y, self.r, DARKPURPLE);

        draw_triangle(
            vec2(self.pos.x - 15., self.pos.y + 55.),
            vec2(self.pos.x + 15., self.pos.y + 55.),
            vec2(self.pos.x, self.pos.y + 25.),
            DARKPURPLE,
        );
    }
}
