use nannou::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub col: u16,
    pub row: u16,
    pub w: u16,
    pub walls: [bool; 4],
    pub visited: bool,
}

impl Cell {
    pub fn new(col: u16, row: u16, w: u16) -> Self {
        let walls = [true, true, true, true];
        let visited = false;
        Self { col, row, w, walls, visited }
    }

    pub fn show(&self, draw: &Draw, win: Rect) {
        // Calculate offsets to center the grid in the window.
        let offset_x = win.w() / 2.0 - self.w as f32 / 2.0;
        let offset_y = win.h() / 2.0 - self.w as f32 / 2.0;
        let wall_offset = self.w as f32 / 2.0;
        let x = (self.col * self.w) as f32 - offset_x;
        let y = (self.row * self.w) as f32 - offset_y;

        // North wall
        if self.walls[0] {
            let start_point = pt2(x - wall_offset, y + wall_offset);
            let end_point   = pt2(x + wall_offset, y + wall_offset);
            draw.line()
                .start(start_point)
                .end(end_point)
                .weight(2.0)
                .color(WHITE);
        }

        // East wall
        if self.walls[1] {
            let start_point = pt2(x + wall_offset, y + wall_offset);
            let end_point   = pt2(x + wall_offset, y - wall_offset);
            draw.line()
                .start(start_point)
                .end(end_point)
                .weight(2.0)
                .color(WHITE);
        }

        // South wall
        if self.walls[2] {
            let start_point = pt2(x - wall_offset, y - wall_offset);
            let end_point   = pt2(x + wall_offset, y - wall_offset);
            draw.line()
                .start(start_point)
                .end(end_point)
                .weight(2.0)
                .color(WHITE);
        }

        // West wall
        if self.walls[3] {
            let start_point = pt2(x - wall_offset, y - wall_offset);
            let end_point   = pt2(x - wall_offset, y + wall_offset);
            draw.line()
                .start(start_point)
                .end(end_point)
                .weight(2.0)
                .color(WHITE);
        }

        if self.visited {
            draw.rect()
                .w_h(self.w as f32, self.w as f32)
                .x_y(x, y)
                .rgba(0.5, 0.0, 0.5, 0.5);
        }
    }
}
