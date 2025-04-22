#![allow(dead_code)]

use macroquad::prelude::*;
use noise::{ Fbm, NoiseFn, Perlin };
use std::collections::HashSet;

struct Walker {
    tx: f64,
    ty: f64,
    x: f64,
    y: f64,
}

impl Walker {
    fn noise_step(&mut self, noise: &Fbm<Perlin>) {
        //println!("X: {}, Y: {}", self.x, self.y);
        self.x = Self::map_range(noise.get([self.tx, 0.0]) as f32, 0.0, 1.0, screen_width() / 2.0, screen_width()) as f64;
        self.y = Self::map_range(noise.get([self.ty, 0.0]) as f32, 0.0, 1.0, screen_height() / 2.0, screen_height()) as f64;

        self.tx += 0.01;
        self.ty += 0.01;

        //println!("Noise X: {}, Noise Y: {}", noise.get([self.tx, 0.0]), noise.get([self.ty, 0.0]));
    }

    fn map_range(
        value: f32,
        in_min: f32,
        in_max: f32,
        out_min: f32,
        out_max: f32,
    ) -> f32 {
        (value - in_min) / (in_max - in_min) * (out_max - out_min) + out_min
    }

    fn show(&self) {
        draw_circle(self.x as f32, self.y as f32, 5.0, WHITE);
    }

    fn pos_key(&self) -> (i32, i32) {
        (self.x.round() as i32, self.y.round() as i32)
    }
}

#[macroquad::main("Random Walker")]
async fn main() {
    let mut walker = Walker { tx: 1.0, ty: 69000.0, x: screen_width() as f64 / 2.0, y: screen_height() as f64 / 2.0 };

    let noise = Fbm::<Perlin>::new(0);

    let mut trail: HashSet<(i32, i32)> = HashSet::new();

    loop {
        clear_background(BLACK);

        walker.noise_step(&noise);
        let pos = walker.pos_key();
        trail.insert(pos);

        for &(x, y) in &trail {
            draw_circle(x as f32, y as f32, 1.0, GRAY);
        }

        walker.show();

        draw_fps();

        next_frame().await
    }
}

