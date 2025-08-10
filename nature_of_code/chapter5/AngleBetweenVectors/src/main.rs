#![allow(unused)]

use macroquad::prelude::*;

#[macroquad::main(window_conf())]
async fn main() {
    loop {
        clear_background(DARKGRAY);

        let scrn_center = Vec2::new(screen_width() / 2., screen_height() / 2.);
        let x_axis = Vec2::new(scrn_center.x + 100., scrn_center.y);
        let mouse_pos = Vec2::new(mouse_position().0, mouse_position().1);
        let mut v = (mouse_pos - scrn_center);
        v = v.normalize();
        v *= 100.;

        draw_line(scrn_center.x, scrn_center.y, x_axis.x, x_axis.y, 5., WHITE);
        draw_line(
            scrn_center.x,
            scrn_center.y,
            scrn_center.x + v.x,
            scrn_center.y + v.y,
            5.,
            WHITE,
        );

        let theta = v.angle_between(x_axis);

        draw_text(&theta.to_string(), 50., 50., 24., WHITE);
        draw_text(&theta.to_degrees().to_string(), 50., 100., 42., WHITE);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("AngleBetweenVectors"),
        ..Default::default()
    }
}

#[allow(unused)]
fn draw_vector(a: &Vec2, b: &Vec2) {
    draw_line(a.x, a.y, b.x, b.y, 5., WHITE);
}
