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

    fn calculate_index(c: i16, r: i16, cols: i16) -> Option<i16> {
        if c < 0 || r < 0 || c >= cols || r >= cols {
            None
        } else {
            Some(c + r * cols)
        }
    }

    pub fn pick_next_index(&self, cols: i16) -> Option<i16> {
        let mut neighbors = vec![];

        if let Some(top) = Cell::calculate_index(self.col, self.row + 1, cols) {
            neighbors.push(top);
        }
        if let Some(right) = Cell::calculate_index(self.col + 1, self.row, cols) {
            neighbors.push(right);
        }
        if let Some(bottom) = Cell::calculate_index(self.col, self.row - 1, cols) {
            neighbors.push(bottom);
        }
        if let Some(left) = Cell::calculate_index(self.col - 1, self.row, cols) {
            neighbors.push(left);
        }

        if neighbors.is_empty() {
            None
        } else {
            let r = random_range(0, neighbors.len());
            Some(neighbors[r])
        }
    }
}
