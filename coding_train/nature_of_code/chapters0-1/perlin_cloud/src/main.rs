use macroquad::prelude::*;
use noise::{ Fbm, Perlin, NoiseFn };

#[macroquad::main("Perlin Cloud")]
async fn main() {
    let width = screen_width();
    let height = screen_height();

    let r = rand::gen_range(0,u32::MAX);
    let noise = Fbm::<Perlin>::new(r);
    let scale = 0.001; 
    // Larger scale produced more 'grainy' noise, smaller scale produces clouds

    let mut t = 0.0;

    let mut texture_data = vec![0u8; (width * height * 4.0) as usize];

    let texture = Texture2D::from_rgba8(width as u16, height as u16, &texture_data);

    texture.set_filter(FilterMode::Linear);

    loop {
        for y in 0..height as usize {
            for x in 0..width as usize {
                let noise_value = noise.get([(x as f64 * scale), (y as f64 * scale), t]);
                let color_value = (noise_value * 0.5 + 0.5) as f32;

                let index: usize = (y * width as usize + x) * 4;

                texture_data[index]     = (color_value * 255.0) as u8; // Red
                texture_data[index + 1] = (color_value * 255.0) as u8; // Green
                texture_data[index + 2] = (color_value * 255.0) as u8; // Blue
                texture_data[index + 3] = 255; // Alpha
            }
        }

        texture.update_from_bytes(width as u32, height as u32, &texture_data);

        clear_background(BLACK);
        draw_texture(&texture, 0.0, 0.0, WHITE);

        t += 0.01;

        draw_fps();

        next_frame().await;
    }
}
