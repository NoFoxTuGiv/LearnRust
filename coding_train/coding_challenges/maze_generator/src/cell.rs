use nannou::prelude::*;

#[derive(Debug)]
pub struct Cell {
    col: u16,
    row: u16,
    w: u16,
}

impl Cell {
    pub fn new(_app: &App, w: u16, col: u16, row: u16) -> Self {
        Self { col, row, w }
    }
}
