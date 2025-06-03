#![allow(unused)]

use macroquad::prelude::*;
// use miniquad::date;
// use noise::{ NoiseFn, Perlin, Fbm };

mod squig;
mod movement;

#[macroquad::main("Ecosystem - Ch 2")]
async fn main() {
    // rand::srand(date::now() as u64);
    // let rseed = rand::gen_range(0, u32::MAX);
    // let noise = Fbm::<Perlin>::new(rseed);

    loop{
        clear_background(BLACK);

        draw_text("Hello, macroquad!", screen_width() / 2. - 100., screen_height() / 2., 24., WHITE);
        next_frame().await;
    }
}
