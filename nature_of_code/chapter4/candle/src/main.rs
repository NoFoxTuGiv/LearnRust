mod emitter;

use macroquad::prelude::*;
use emitter::Emitter;

#[macroquad::main(window_conf())]
async fn main() {
    let mut emitter = Emitter::new();
    loop {
        clear_background(DARKGRAY);

        emitter.add_particle();
        emitter.run();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: ("Candle by NoFoxTuGiv").into(),
        ..Default::default()
    }
}
