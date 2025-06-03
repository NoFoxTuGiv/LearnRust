use macroquad::prelude::*;
use noise::{ Fbm, NoiseFn, Perlin};
use miniquad::date;

#[macroquad::main("Perlin Noise Graph")]
async fn main() {
    rand::srand(date::now() as u64);

    let r = rand::gen_range(0,u32::MAX);
    let noise = Fbm::<Perlin>::new(r);

    let width = screen_width();
    let height = screen_height();

    let mut t = 0.0;
    let x_scale = 0.005;
    let y_scale = height / 2.5;
    let y_offset = height / 2.0;

    loop {
        clear_background(BLACK);

        let mut last_x = 0.0;
        let mut last_y = 0.0;

        for x in 0..width as usize {
            let nx = x as f32 * x_scale;

            let y: f32 = noise.get([(nx + t) as f64, 0.0]) as f32 * y_scale + y_offset;

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
