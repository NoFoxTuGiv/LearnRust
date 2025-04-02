use nannou::prelude::*;
use nannou::noise::Perlin;

struct Model{
    x_off_a: usize,
    x_off_b: usize,
}

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

fn model(_app: &App) -> Model {
    let x_off_a = 0;
    let x_off_b = 1000;

    Model{ x_off_a, x_off_b }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.x_off_a += 1;
    model.x_off_b += 1;
}

fn view(app: &App, model: &Model, _frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
}
