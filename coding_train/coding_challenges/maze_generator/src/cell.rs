use nannou::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub col: u16,
    pub row: u16,
    pub w: u16,
}

impl Cell {
    pub fn new(col: u16, row: u16, w: u16) -> Self {
        Self { col, row, w }
    }

    pub fn show(&self, app: &App) {
        let draw = app.draw();
        let win = app.window_rect();
        // Calculate offsets to center the grid in the window.
        let offset_x = win.w() as f32 / 2.0 - self.w as f32 / 2.0;
        let offset_y = win.h() as f32 / 2.0 - self.w as f32 / 2.0;
        draw.rect()
            .w_h(self.w as f32, self.w as f32)
            .x_y((self.col * self.w) as f32 - offset_x, (self.row * self.w) as f32 - offset_y)
            .stroke(WHITE)
            .no_fill();
    }

    //pub fn show(self: &Self, app: &App) {
    //    let draw = app.draw();
    //    draw.rect()
    //        .w_h(self.w.into(), self.w.into())
    //        .x_y((self.col * self.w).into(), (self.row * self.w).into())
    //        .stroke(WHITE)
    //        .no_fill();
    //}
}
