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

    pub fn show(&self, draw: &Draw, win: Rect) {
        // Calculate offsets to center the grid in the window.
        let offset_x = win.w() / 2.0 - self.w as f32 / 2.0;
        let offset_y = win.h() / 2.0 - self.w as f32 / 2.0;
        let cell_size = self.w as f32;
        let x = (self.col * self.w) as f32 - offset_x;
        let y = (self.row * self.w) as f32 - offset_y;

        // North wall
        let start_point = pt2(x - cell_size / 2.0, y + cell_size / 2.0);
        let end_point   = pt2(x + cell_size / 2.0, y + cell_size / 2.0);
        draw.line()
            .start(start_point)
            .end(end_point)
            .weight(2.0)
            .color(WHITE);

        // East wall
        let start_point = pt2(x + cell_size / 2.0, y + cell_size / 2.0);
        let end_point   = pt2(x + cell_size / 2.0, y - cell_size / 2.0);
        draw.line()
            .start(start_point)
            .end(end_point)
            .weight(2.0)
            .color(WHITE);

        // South wall
        let start_point = pt2(x - cell_size / 2.0, y - cell_size / 2.0);
        let end_point   = pt2(x + cell_size / 2.0, y - cell_size / 2.0);
        draw.line()
            .start(start_point)
            .end(end_point)
            .weight(2.0)
            .color(WHITE);

        // West wall
        let start_point = pt2(x - cell_size / 2.0, y - cell_size / 2.0);
        let end_point   = pt2(x - cell_size / 2.0, y + cell_size / 2.0);
        draw.line()
            .start(start_point)
            .end(end_point)
            .weight(2.0)
            .color(WHITE);



        //draw.rect()
        //    .w_h(cell_size, cell_size)
        //    .x_y(x, y)
        //    .stroke_color(WHITE)
        //    .stroke_weight(2.0)
        //    .no_fill();
    }
}
