mod star;

use nannou::prelude::*;
use star::Star;

struct Model {
    stars: [Star; 800],
}

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

fn model(app: &App) -> Model {
    let stars = [Star::new(app); 800];
    Model { stars }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    for star in &mut model.stars {
        star.update(app);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    for star in &model.stars {
        star.show(app, &draw);
    }

    draw.to_frame(app, &frame).unwrap();
}
