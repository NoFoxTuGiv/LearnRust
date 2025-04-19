#![allow(dead_code)]
mod star;

use macroquad::prelude::*;
use star::Star;

#[macroquad::main("Starfield")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);

    let delta_time = get_frame_time();

    let mut stars: Vec<Star> = vec![];

    for _ in 0..300 {
        let star = Star {
            x: rand::gen_range(-screen_width() / 2.0, screen_width() / 2.0),
            y: rand::gen_range(-screen_width() / 2.0, screen_height() / 2.0),
            z: rand::gen_range(0.0, screen_width() / 2.0),
        };

        stars.push(star);
    }

    loop {
        clear_background(BLACK);

        for star in &mut stars {
            star.update(delta_time);
            star.show();
        }

        next_frame().await
    }
}
