#![allow(unused)]

use macroquad::prelude::*;

#[macroquad::main(window_conf())]
async fn main() {
    loop {
        clear_background(DARKGRAY);

        let scrn_center = Vec2::new(screen_width() / 2., screen_height() / 2.);
        let x_axis = Vec2::new(100., 0.);
        let mouse_pos = Vec2::new(mouse_position().0, mouse_position().1);
        let mut v = set_mag(mouse_pos - scrn_center, 100.);

        draw_line(
            scrn_center.x,
            scrn_center.y,
            scrn_center.x + x_axis.x,
            scrn_center.y + x_axis.y,
            5.,
            WHITE);
        draw_line(
            scrn_center.x,
            scrn_center.y,
            scrn_center.x + v.x,
            scrn_center.y + v.y,
            5.,
            WHITE,
        );
        draw_ellipse(scrn_center.x, scrn_center.y, 5., 5., 0., WHITE);

        let theta = v.angle_between(x_axis);
        let theta_text = format!("{:.2} radians", theta);
        draw_text(&theta_text, 50., 50., 24., WHITE);

        let degrees = theta.to_degrees() as i16;
        let degrees_text = format!("°{}", degrees);
        draw_text(&degrees_text, 50., 100., 42., WHITE);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("AngleBetweenVectors"),
        window_width: 400,
        window_height: 360,
        window_resizable: false,
        ..Default::default()
    }
}

fn set_mag(vec: Vec2, scalar: f32) -> Vec2 {
    vec.normalize() * scalar
}

/// Draws an arrow pointing from a to b
///
/// # Parameters
#[allow(unused)]
fn draw_vertex(a: Vec2, b: Vec2) {
    draw_line(a.x, a.y, b.x, b.y, 5., WHITE);
}
