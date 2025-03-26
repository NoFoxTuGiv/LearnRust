use nannou::prelude::*;
// use nannou::rand::random_range;

pub struct Cell {
    col: u16,
    row: u16,
    w: u16,
}

impl Cell {
    pub fn new(&app: App) -> Self {
        Self { col, row, w }
    }
}