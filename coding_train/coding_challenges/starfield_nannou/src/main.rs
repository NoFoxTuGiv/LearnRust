mod star;

use nannou::prelude::*;
use star::Star;

struct Model {
    stars: [Star; 800],
}

fn main() {
    nannou::app(model)
        .update(update)
        // TODO: Figure out fps...
        //.loop_mode(LoopMode::rate_fps(60.0))
        .simple_window(view)
        .run();
} 

fn model(app: &App) -> Model {
    //app.set_loop_mode(LoopMode::rate_fps(60.0));
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
