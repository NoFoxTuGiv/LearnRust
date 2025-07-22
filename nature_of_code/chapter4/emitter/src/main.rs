mod emitter;

use emitter::Emitter;
use macroquad::prelude::*;

#[macroquad::main(window_conf())]
async fn main() {
    // === Setup ===
    let mut emitter = Emitter::new();

    loop {
        clear_background(DARKGRAY);

        // == Draw loop ===
        emitter.add_particle();
        emitter.run();

        // == Update position to mouse pos ==
        if is_mouse_inside_window() {
            let m_pos: Vec2 = mouse_position().into();
            emitter.update_origin(m_pos);
        }

        // == Exit conditions ==
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        // == Debug ==
        #[cfg(debug_assertions)]
        {
            draw_fps();
        }

        next_frame().await;
    }
}

fn is_mouse_inside_window() -> bool {
    let (x, y) = mouse_position();
    x >= 0.0 && x <= screen_width() && y >= 0.0 && y <= screen_height()
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Particle Emitter by NoFoxTuGiv".into(),
        window_width: 600,
        window_height: 400,
        ..Default::default()
    }
}
