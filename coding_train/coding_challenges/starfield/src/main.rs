#![allow(dead_code)]

mod star;

use nannou::prelude::*;
use star::Star;

struct Model {
    _window: window::Id,
    stars: Vec<Star>,
}

fn main() {
    nannou::app(model)
        .update(update)
        .view(view)
        .run();
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    let stars: Vec<Star> = (0..100).map(|_| Star::new(&app)).collect();
    Model { _window, stars }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    for star in &mut model.stars {
        star.update(app);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    //TODO: Draw the fukken stars

    for star in &model.stars {
        star.show(app, &draw);
    }

    draw.to_frame(app, &frame).unwrap();
}
