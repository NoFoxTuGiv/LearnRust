#![allow(dead_code)]
mod cell;

use nannou::prelude::*;
use cell::Cell;

const WIDTH: u16 = 800;
const HEIGHT: u16 = 600;
// const COLS: u16;
// const ROWS: u16;

struct Model {
    window: window::Id,
}

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

fn model(app: &App) -> Model {
    let window = app.new_window()
    .size(WIDTH.into(), HEIGHT.into())
    .view(view)
    .build()
    .unwrap();
    Model { window }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    draw.to_frame(app, &frame).unwrap();
}