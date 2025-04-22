use macroquad::prelude::*;
use noise::{ Fbm, NoiseFn, Perlin};

#[macroquad::main("Perlin Noise Graph")]
async fn main() {
    let r = rand::gen_range(0,100);
    let noise = Fbm::<Perlin>::new(r);
    let width = screen_width();
    let height = screen_height();
    let y_scale = height / 2.5;
    let x_scale = 0.01;
    let y_offset = height / 2.0;
    let octaves = 3;
    let persistence = 0.5;
    let mut t = 0.0;

    loop {
        clear_background(BLACK);

        let mut last_x = 0.0;
        let mut last_y = 0.0;

        for x in 0..width as usize {
            let nx = x as f32 * x_scale;
            let mut noise_val = 0.0;
            let mut frequency = 1.0;
            let mut amplitude = 1.0;

            for _ in 0..octaves {
                noise_val += noise.get([(nx as f64 + t) * frequency, 0.0]) as f32 * amplitude;
                frequency *= 2.0;
                amplitude *= persistence;
            }

            let y = noise_val * y_scale + y_offset;

            if x > 0 {
                draw_line(last_x, last_y, x as f32, y, 2.0, WHITE);
            }

            last_x = x as f32;
            last_y = y;
        }

        t += 0.01;
        next_frame().await;
    }
}
