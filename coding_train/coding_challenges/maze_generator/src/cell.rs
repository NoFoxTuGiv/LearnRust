use nannou::prelude::*;
use nannou::rand::random_range;

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

    fn calculate_index(c: i16, r: i16, cols: i16) -> i16 {
        if c < 0 || r < 0 || c > cols || r > cols {
            return -1;
        }
        return c + r * cols;
    }

    pub fn pick_next_index(&self, cols: i16) -> i16 {
        let mut neighbors: [i16; 4] = [-1; 4];

        let top = Cell::calculate_index(self.col.into(), self.row + 1, cols);
        if top > -1 {
            neighbors[0] = top;
        }
        let right = Cell::calculate_index(self.col + 1, self.row, cols);
        if right > -1 {
            neighbors[1] = right;
        }
        let bottom = Cell::calculate_index(self.col, self.row - 1, cols);
        if bottom > -1 {
            neighbors[2] = bottom;
        }
        let left = Cell::calculate_index(self.col - 1, self.row, cols);
        if left > -1 {
            neighbors[3] = left;
        }

        let r = random_range(0, 4);

        return neighbors[r];
    }
}
