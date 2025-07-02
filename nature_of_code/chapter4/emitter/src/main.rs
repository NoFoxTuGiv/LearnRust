mod emitter;

use emitter::{Emitter};
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
        let m_pos: Vec2 = mouse_position().into();
        emitter.update_origin(m_pos);

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

fn window_conf() -> Conf {
    Conf {
        window_title: "Particle Emitter by NoFoxTuGiv".into(),
        ..Default::default()
    }
}
