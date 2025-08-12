use macroquad::prelude::*;

#[macroquad::main(window_conf())]
async fn main() {
    loop {
        clear_background(DARKGRAY);

        let scrn_center = Vec2::new(screen_width() / 2., screen_height() / 2.);
        let x_axis = Vec2::new(100., 0.);
        let mouse_pos = Vec2::new(mouse_position().0, mouse_position().1);
        let v = set_mag(mouse_pos - scrn_center, 100.);

        draw_angle_arc(scrn_center, x_axis, v, 0.25, 24, 2., MAGENTA);
        draw_vertex(scrn_center, x_axis, BLUE);
        draw_vertex(scrn_center, v, GREEN);

        let theta = v.angle_between(x_axis);
        let theta_text = format!("{:.2} radians", theta);
        draw_text(&theta_text, 30., 30., 24., WHITE);

        let degrees = theta.to_degrees() as i16;
        let degrees_text = format!("°{}", degrees);
        draw_text(&degrees_text, 30., 60., 24., WHITE);

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
        window_height: 300,
        window_resizable: false,
        ..Default::default()
    }
}

fn set_mag(vec: Vec2, scalar: f32) -> Vec2 {
    vec.normalize() * scalar
}

fn draw_vertex(a: Vec2, b: Vec2, color: Color) {
    const ARROW_R: f32 = 10.;
    const ARROW_A: f32 = 30_f32.to_radians();

    let tip = a + b;
    draw_line(a.x, a.y, tip.x, tip.y, 2., color);

    let theta = b.normalize().to_angle();

    let left_vec = Vec2::from_angle(theta + ARROW_A) * ARROW_R;
    let right_vec = Vec2::from_angle(theta - ARROW_A) * ARROW_R;

    draw_line(
        tip.x,
        tip.y,
        (tip - left_vec).x,
        (tip - left_vec).y,
        2.,
        color,
    );
    draw_line(
        tip.x,
        tip.y,
        (tip - right_vec).x,
        (tip - right_vec).y,
        2.,
        color,
    );
}

fn draw_angle_arc(
    center: Vec2,
    v_from: Vec2,
    v_to: Vec2,
    radius_factor: f32,
    segments: usize,
    thickness: f32,
    color: Color,
) {
    use std::f32::consts::PI;

    let r = radius_factor * v_from.length().min(v_to.length());

    // start/end angles from the vectors
    let a0 = v_from.normalize().to_angle();
    let a1 = v_to.normalize().to_angle();

    // unwrap delta to match the signed/shortest angular difference
    let mut delta = a1 - a0;
    while delta > PI {
        delta -= 2.0 * PI;
    }
    while delta < -PI {
        delta += 2.0 * PI;
    }

    // draw the arc as small line segments
    for i in 0..segments {
        let t0 = a0 + delta * (i as f32 / segments as f32);
        let t1 = a0 + delta * ((i + 1) as f32 / segments as f32);
        let p0 = center + Vec2::from_angle(t0) * r;
        let p1 = center + Vec2::from_angle(t1) * r;
        draw_line(p0.x, p0.y, p1.x, p1.y, thickness, color);
    }
}
