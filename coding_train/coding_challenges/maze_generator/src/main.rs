#![allow(dead_code)]
mod cell;

use cell::Cell;
use nannou::prelude::*;

const WIDTH: i16 = 800;
const HEIGHT: i16 = 800;
const CELL_WIDTH: i16 = 40;
const COLS: i16 = WIDTH / CELL_WIDTH;
const ROWS: i16 = HEIGHT / CELL_WIDTH;
const N: usize = (COLS * ROWS) as usize;

struct Model {
    window: window::Id,
    cells: [Cell; N],
    current_cell: usize,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let window = app
        .new_window()
        .size(WIDTH as u32, HEIGHT as u32)
        .view(view)
        .build()
        .unwrap();
    
    let mut cells: [Cell; N] = [Cell::new(0, 0, CELL_WIDTH); N];

    for row in 0..ROWS {
        for col in 0..COLS {
            let cell = Cell::new(col, row, CELL_WIDTH);
            cells[((row * COLS) + col) as usize] = cell
        }
    }
    
    let current_cell = 0;

    Model { window, cells, current_cell }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.cells[model.current_cell].visited = true;

    let next_index = model.cells[model.current_cell].pick_next_index(COLS);
    if next_index > -1 && !model.cells[next_index as usize].visited {
        model.current_cell = next_index as usize;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    draw.background().color(BLACK);

    for cell in model.cells.iter() {
        cell.show(&draw, win);
    }

    draw.to_frame(app, &frame).unwrap();
}
