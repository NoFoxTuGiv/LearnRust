use macroquad::prelude::*;

#[macroquad::main(window_conf())]
async fn main() {
    let mut radius: f32 = 0.01;
    let mut theta: f32 = 0.01;
    let mut outward = true;
    let offset = vec2(screen_width() / 2., screen_height() / 2.);
    let theta_c = 0.05;

    loop {
        clear_background(DARKGRAY);

        // == trail
        // pass screenshot from last frame in as uniform material
        // use gl shader
        // draw texture ex with shader
        // use default shader
        // get screenshot of frame for next frame

        // let x = r * f32::cos(theta) + offset.x;
        // let y = r * f32::sin(theta) + offset.y;

        // draw_circle(x, y, 12., Color::from_rgba(180, 0, 140, 255));
        draw_trail(radius, theta, offset);

        if radius < screen_height() * 0.4 && outward {
            theta -= theta_c;
            radius += 0.05;
            if radius > screen_height() * 0.39 {
                outward = !outward;
            }
        } else if radius > 0.1 && !outward {
            theta -= theta_c;
            radius -= 0.05;
            if radius < 0.11 {
                outward = !outward;
            }
        }

        // raylib style exit
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Spiral".to_owned(),
        ..Default::default()
    }
}

fn draw_trail(radius: f32, theta: f32, offset: Vec2) {
    let someshit = 0.01;

    for i in 0..500 {
        let alpha = 1. - (i as f32 * 0.01);
        let temp_r = radius - (alpha * 5.);
        let temp_theta = theta - (alpha * (someshit * (1. / someshit)));
        let x = temp_r * f32::cos(temp_theta) + offset.x;
        let y = temp_r * f32::sin(temp_theta) + offset.y;

        draw_circle(x, y, 12., Color::from_rgba(180, 0, 140, (alpha * 255.) as u8));
    }
}
