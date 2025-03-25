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
        Self {
            x: random_range(win.left(), win.right()),
            y: random_range(win.bottom(), win.top()),
            z: random_range(0.0, win.w()),
        }
    }
}
