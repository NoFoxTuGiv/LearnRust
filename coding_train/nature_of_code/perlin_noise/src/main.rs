#![allow(dead_code)]

use macroquad::prelude::*;

struct Walker {
    x: f32,
    y: f32,
}

impl Walker {
    fn show(&self) {
        draw_circle(self.x, self.y, 5.0, WHITE);
    }

    fn step(&mut self) {
        let choice = rand::gen_range(0, 4);

        match choice {
            0 => self.x += 1.0,
            1 => self.x -= 1.0,
            2 => self.y += 1.0,
            3 => self.y -= 1.0,
            _ => panic!("Choice selection out of range during step() function."),
        }
    }
}

#[macroquad::main("")]
async fn main() {
    loop {
        clear_background(BLACK);

        let walker = Walker { x:screen_width() / 2.0, y:screen_height() / 2.0 };

        walker.show();

        next_frame().await
    }
}
