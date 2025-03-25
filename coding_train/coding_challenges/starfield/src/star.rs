#![allow(dead_code)]

use nannou::prelude::*;
use nannou::rand::random_range;

pub struct Star {
    x: f32,
    y: f32,
    z: f32,
}

impl Star {
    pub fn new(app: &App) -> Self {
        let win = app.window_rect();
        let x = random_range(win.left(), win.right());
        let y = random_range(win.bottom(), win.top());
        let z = random_range(0.0, win.w());

        Self { x, y, z }
    }

    pub fn update(&mut self, _app: &App) {}

    pub fn show(self: &Self, _app: &App, draw: &Draw) {
        draw.ellipse()
            .x_y(self.x, self.y)
            .radius(10.0)
            .color(WHITE);
    }
}
