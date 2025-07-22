mod vehicle;

use macroquad::prelude::*;
use vehicle::Vehicle;

#[macroquad::main(window_conf)]
async fn main() {
    rand::srand(miniquad::date::now() as u64);

    let mut vehicle: Vehicle = Vehicle::new();
    let mut debug: bool = false;
    let mut seeking: bool = false;
    let mut fleeing: bool = false;

    let mut seek_force = Vec2::ZERO;
    let mut flee_force = Vec2::ZERO;
    let mut target = Vec2::ZERO;

    loop {
        clear_background(DARKGRAY);

        // --- Target Shit ---
        if seeking {
            target = Vec2::new(mouse_position().0, mouse_position().1);
            seek_force = vehicle.seek(&target);
        } else if fleeing {
            let threat = Vec2::new(mouse_position().0, mouse_position().1);
            flee_force = vehicle.flee(&threat);
        } else {
            target = Vec2::ZERO;
            seek_force = Vec2::ZERO;
            flee_force = Vec2::ZERO;
        }
        let boundary_force = vehicle.boundary_force(45.);
        vehicle.apply_force(seek_force + boundary_force + flee_force);

        // --- Vehicle Shit ---
        vehicle.update();

        // --- Debug Drawing ---
        if debug {
            draw_text("DEBUG".into(), 10., 20., 24., GRAY);
            let seek_text = format!("Seeking: {}", seeking);
            let flee_text = format!("Fleeing: {}", fleeing);

            draw_text(&seek_text, 10., 50., 24., GRAY);
            draw_text(&flee_text, 10., 80., 24., GRAY);

            // Draw Target
            let v_pos = vehicle.get_position();
            let scale = 500.;

            draw_ellipse(target.x, target.y, 10., 10., 0., GRAY);

            // Draw Seek Vector
            draw_line(
                v_pos.x,
                v_pos.y,
                v_pos.x + seek_force.x * scale,
                v_pos.y + seek_force.y * scale,
                2.,
                GREEN,
            );

            // Draw boundary vectors
            draw_line(
                v_pos.x,
                v_pos.y,
                v_pos.x + boundary_force.x * scale,
                v_pos.y + boundary_force.y * scale,
                2.,
                BLUE,
            );
        } else {
            draw_text("Press D for Debug".into(), 10., 20., 24., GRAY);
            draw_text("Press S for Seeking".into(), 10., 50., 24., GRAY);
            draw_text("Press F for Fleeing".into(), 10., 80., 24., GRAY);
        }
        draw_text(
            "Press Esc to Exit.".into(),
            10.,
            screen_height() - 30.,
            24.,
            GRAY,
        );

        // Vehicle Drawing
        vehicle.show();

        // --- Raylib Style Exit ---
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_key_pressed(KeyCode::D) {
            debug = !debug;
        }

        if is_key_pressed(KeyCode::S) {
            seeking = !seeking;
            fleeing = false;
        }

        if is_key_pressed(KeyCode::F) {
            fleeing = !fleeing;
            seeking = false;
        }

        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Vehicles and Steering by NoFoxTuGiv".into(),
        ..Default::default()
    }
}
