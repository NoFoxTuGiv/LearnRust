use nannou::prelude::*;

pub struct Cell {
    col: u16,
    row: u16,
    w: u16,
}

impl Cell {
    pub fn new(app: &App) -> Self {
        //placeholder values
        let col = 0;
        let row = 0;
        let w = 0;
        Self { col, row, w }
    }
}