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
    stack: Vec<usize>,
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

    let stack: Vec<usize> = Vec::new();

    Model {
        window,
        cells,
        current_cell,
        stack,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.cells[model.current_cell].visited = true;

    let current_col = model.cells[model.current_cell].col;
    let current_row = model.cells[model.current_cell].row;

    let mut neighbors = Vec::new();

    // Check top neighbor
    if let Some(index) = Cell::calculate_index(current_col, current_row + 1, COLS) {
        let idx = index as usize;
        if !model.cells[idx].visited {
            neighbors.push(idx);
        }
    }

    // Check right neighbor
    if let Some(index) = Cell::calculate_index(current_col + 1, current_row, COLS) {
        let idx = index as usize;
        if !model.cells[idx].visited {
            neighbors.push(idx);
        }
    }

    // Check bottom neighbor
    if let Some(index) = Cell::calculate_index(current_col, current_row - 1, COLS) {
        let idx = index as usize;
        if !model.cells[idx].visited {
            neighbors.push(idx);
        }
    }

    // Check left neighbor
    if let Some(index) = Cell::calculate_index(current_col - 1, current_row, COLS) {
        let idx = index as usize;
        if !model.cells[idx].visited {
            neighbors.push(idx);
        }
    }

    if !neighbors.is_empty() {
        let next_idx = neighbors[random_range(0, neighbors.len())];
        remove_walls(&mut model.cells, model.current_cell, next_idx);
        model.stack.push(model.current_cell);
        model.current_cell = next_idx;
    } else if !model.stack.is_empty() {
        model.current_cell = model.stack.pop().unwrap();
        //println!("Backtracking to cell number {}", model.current_cell);
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
    let x = cells[a_index].col - cells[b_index].col;

    if x == 1 {
        let a = &mut cells[a_index];
        a.walls[3] = false;
        let b = &mut cells[b_index];
        b.walls[1] = false;
    } else if x == -1 {
        let a = &mut cells[a_index];
        a.walls[1] = false;
        let b = &mut cells[b_index];
        b.walls[3] = false;
    }

    let y = cells[a_index].row - cells[b_index].row;

    if y == -1 {
        let a = &mut cells[a_index];
        a.walls[0] = false;
        let b = &mut cells[b_index];
        b.walls[2] = false;
    } else if y == 1 {
        let a = &mut cells[a_index];
        a.walls[2] = false;
        let b = &mut cells[b_index];
        b.walls[0] = false;
    }
}
