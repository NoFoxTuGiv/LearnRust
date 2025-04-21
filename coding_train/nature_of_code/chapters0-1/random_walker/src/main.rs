use std::collections::HashSet;
use macroquad::prelude::*;

struct Walker {
    pos: Vec2,
}

impl Walker {
    fn show(&self) {
        draw_circle(self.pos.x, self.pos.y, 5.0, WHITE);
    }

    fn step(&mut self) {
        let x: f32 = rand::gen_range(-1.0, 1.0);
        let y: f32 = rand::gen_range(-1.0,  1.0);

        let step = Vec2::new(x, y);

        self.pos += step;
    }

    fn pos_key(&self) -> (i32, i32) {
        (self.pos.x.round() as i32, self.pos.y.round() as i32)
    }
}

#[macroquad::main("Random Walker")]
async fn main() {
    let mut walker = Walker { pos: vec2(screen_width() / 2.0, screen_height() / 2.0)};

    let mut trail: HashSet<(i32, i32)> = HashSet::new();

    loop {
        clear_background(BLACK);

        walker.step();
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

