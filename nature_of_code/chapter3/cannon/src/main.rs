use macroquad::prelude::*;

#[macroquad::main("Cannon")]
async fn main() {
    loop {
        clear_background(DARKGRAY);

        next_frame().await;
    }
}
