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

    Model {
        window,
        cells,
        current_cell,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.cells[model.current_cell].visited = true;

    let next_index = model.cells[model.current_cell].pick_next_index(COLS);
    if let Some(next_index) = next_index {
        if !model.cells[next_index as usize].visited {
            remove_walls(&mut model.cells, model.current_cell, next_index as usize);
            model.current_cell = next_index as usize;
        }
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

fn remove_walls(cells: &mut [Cell; N], a_index: usize, b_index: usize) {
    // let (a, b) = (&mut cells[a_index], &mut cells[b_index]);
    // let x = a.col - b.col;
    // let y = a.row - b.row;

    let x = cells[a_index].col - cells[b_index].col;
    let y = cells[a_index].row - cells[b_index].row;

    if x == 1 {
        let a = (&mut cells[a_index]);
        a.walls[3] = false;
    } else if x == -1 {
        let a = (&mut cells[a_index]);
        a.walls[1] = false;
    }

    if x == 1 {
        let b = (&mut cells[b_index]);
        b.walls[1] = false;
    } else if x == -1 {
        let b = (&mut cells[b_index]);
        b.walls[3] = false;
    }

    if y == 1 {
        let a = (&mut cells[a_index]);
        a.walls[0] = false;
    } else if y == -1 {
        let a = (&mut cells[a_index]);
        a.walls[2] = false;
    }

    if y == 1 {
        let b = (&mut cells[b_index]);
        b.walls[2] = false;
    } else if y == -1 {
        let b = (&mut cells[b_index]);
        b.walls[0] = false;
    }
}
