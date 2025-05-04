use nannou::prelude::*;
use nannou::rand::random_range;

#[derive(Copy, Clone)]
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
        let z = random_range(0.0, win.h());
        // Set this once at the beginning for each star.
        Self { x, y, z }
    }

    pub fn update(&mut self, app: &App) {
        let win = app.window_rect();
        self.z -= 30.0;
        if self.z < 0.1 {
            self.z = random_range(0.0, win.h());
            self.x = random_range(win.left(), win.right());
            self.y = random_range(win.bottom(), win.right());
        }
    }

    pub fn show(self: &Self, app: &App, draw: &Draw) {
        let win = app.window_rect();
        let sx: f32 = map_range(self.x / self.z, 0.0, 1.0, 0.0, win.w());
        let sy: f32 = map_range(self.y / self.z, 0.0, 1.0, 0.0, win.h());
        let r: f32 = map_range(self.z, 0.0, win.w(), 8.0, 0.01);

        draw.ellipse().x_y(sx, sy).radius(r).color(WHITE);
    }
}
