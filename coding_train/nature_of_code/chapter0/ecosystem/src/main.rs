#![allow(unused)]

use macroquad::prelude::*;
use miniquad::date;
use noise::{ NoiseFn, Perlin, Fbm };

struct Squig {
    pub pos: Vec2,
    pub radius: f32,
    p_offset: f64,
}

impl Squig {
    pub fn new(pos: Vec2) -> Squig {
        let r: f32 = rand::gen_range(4., 20.);
        let off = rand::gen_range(0., f64::MAX);
        Squig {
            pos,
            radius: r,
            p_offset: off,
        }
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.radius, PURPLE);
    }

    pub fn walk(&mut self, noise: &Fbm<Perlin>, t: &f64) {
        // TODO:
        // - Add behavioral vectors based on edge and other squigs
        // - Step vector is sum of decision based vectors
        let mult: f64 = rand::gen_range(0., 3.);
        let offset = self.p_offset % 69696969.;
        let x = (noise.get([*t + offset,      0.]) * mult);
        let y = (noise.get([*t + offset / 2., 0.]) * mult);
        let step = Vec2::new(x as f32, y as f32);
        self.pos += step;
        self.pos = Squig::edges(&self.pos);
        // DEBUG
        //dbg!(step);
    }
    //TODO:
    // - Clamp to screen
    fn edges(pos: &Vec2) -> Vec2 {
        let x = (pos.x + screen_width()) % screen_width();
        let y = (pos.y + screen_height()) % screen_height();
        Vec2::new(x, y)
    }
}

#[macroquad::main("Ecosystem - Ch 0")]
async fn main() {
    rand::srand(date::now() as u64);
    let rseed = rand::gen_range(0, u32::MAX);
    let noise = Fbm::<Perlin>::new(rseed);

    let mut squigs: Vec<Squig> = vec![];

    let mut t = 0.;

    for _ in 0..20 {
        let x: f32 = rand::gen_range(20., screen_width() - 20.);
        let y: f32 = rand::gen_range(20., screen_height() - 20.);
        let squig = Squig::new(Vec2::new(x, y));
        squigs.push(squig);
    }

    loop{
        clear_background(BLACK);

        for squig in &mut squigs {
            squig.walk(&noise, &t);
            squig.draw();
        }

        t += 0.01;

        next_frame().await;
    }
}
