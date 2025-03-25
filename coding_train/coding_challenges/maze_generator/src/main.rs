use nannou::prelude::*;

struct Model {}

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw  app.draw();

    draw.background().color(BLACK);

    draw.to_frame(app, &frame).unwrap();
}