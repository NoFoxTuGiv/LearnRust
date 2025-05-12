#![allow(unused)]

use macroquad::prelude::*;

#[macroquad::main("Friction")]
async fn main() {
    loop {
        clear_background(BLACK);

        next_frame().await;
    }
}
