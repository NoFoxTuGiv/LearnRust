#![allow(dead_code)]
mod cell;

use cell::Cell;
use nannou::prelude::*;

const WIDTH: u16 = 800;
const HEIGHT: u16 = 800;
const CELL_WIDTH: u16 = 40;
const COLS: u16 = WIDTH / CELL_WIDTH;
const ROWS: u16 = HEIGHT / CELL_WIDTH;
const N: usize = (COLS * ROWS) as usize;

struct Model {
    window: window::Id,
    cells: [Cell; N],
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let window = app
        .new_window()
        .size(WIDTH.into(), HEIGHT.into())
        .view(view)
        .build()
        .unwrap();
    
    let mut cells_ary: [Cell; N] = [Cell::new(0, 0, CELL_WIDTH); N];

    for row in 0..ROWS {
        for col in 0..COLS {
            let cell = Cell::new(col, row, CELL_WIDTH);
            cells_ary[((row * COLS) + col) as usize] = cell
        }
    }

    Model { window, cells: cells_ary }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    draw.background().color(BLACK);

    for cell in model.cells.iter() {
        cell.show(&draw, win);
    }

    draw.to_frame(app, &frame).unwrap();
}
