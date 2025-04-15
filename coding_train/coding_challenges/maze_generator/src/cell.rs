use nannou::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub col: i16,
    pub row: i16,
    pub w: i16,
    pub walls: [bool; 4],
    pub visited: bool,
}

impl Cell {
    pub fn new(col: i16, row: i16, w: i16) -> Self {
        let walls = [true, true, true, true];
        let visited = false;
        Self {
            col,
            row,
            w,
            walls,
            visited,
        }
    }

    pub fn show(&self, draw: &Draw, win: Rect) {
        // Calculate offsets to center the grid in the window.
        let offset_x = win.w() / 2.0 - self.w as f32 / 2.0;
        let offset_y = win.h() / 2.0 - self.w as f32 / 2.0;
        let wall_offset = self.w as f32 / 2.0;
        let x = (self.col * self.w) as f32 - offset_x;
        let y = (self.row * self.w) as f32 - offset_y;

        let directions = [
            (
                pt2(x - wall_offset, y + wall_offset),
                pt2(x + wall_offset, y + wall_offset),
            ), // North
            (
                pt2(x + wall_offset, y + wall_offset),
                pt2(x + wall_offset, y - wall_offset),
            ), // East
            (
                pt2(x - wall_offset, y - wall_offset),
                pt2(x + wall_offset, y - wall_offset),
            ), // South
            (
                pt2(x - wall_offset, y - wall_offset),
                pt2(x - wall_offset, y + wall_offset),
            ), // West
        ];

        for (i, &(start, end)) in directions.iter().enumerate() {
            if self.walls[i] {
                draw.line().start(start).end(end).weight(0.5).color(WHITE);
            }
        }

        if self.visited {
            draw.rect()
                .w_h(self.w as f32, self.w as f32)
                .x_y(x, y)
                .rgba(0.5, 0.0, 0.5, 0.5);
        }
    }

    pub fn calculate_index(c: i16, r: i16, cols: i16) -> Option<i16> {
        if c < 0 || r < 0 || c >= cols || r >= cols {
            None
        } else {
            Some(c + r * cols)
        }
    }
}
