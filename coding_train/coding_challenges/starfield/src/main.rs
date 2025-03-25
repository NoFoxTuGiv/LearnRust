mod star;

use nannou::prelude::*;
use star::Star;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    let stars: Vec<Star> = (0..100).map(|_| Star::new(&app)).collect();

    //TODO: Draw the fukken stars

    draw.to_frame(app, &frame).unwrap();
}
