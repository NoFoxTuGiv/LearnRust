use macroquad::prelude::*;

#[derive(Copy, Clone)]
pub struct Star {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Star {
    pub fn update(&mut self, delta_time: f32) {
        self.z -= 300.0 * delta_time;
        if self.z < 0.1 {
            self.z = rand::gen_range(0.0, screen_height());
            self.x = rand::gen_range(-screen_width() / 2.0, screen_width());
            self.y = rand::gen_range(-screen_height() / 2.0, screen_height());
        }
    }

    pub fn show(&mut self) {
        let sx = (self.x / self.z) * screen_width() / 2.0 + screen_width() / 2.0;
        let sy = (self.y / self.z) * screen_height() / 2.0 + screen_height() / 2.0;
        let r: f32 = Self::map_range(self.z, 0.0, screen_width(), 8.0, 0.01);
        draw_circle(sx, sy, r, WHITE);
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
}
