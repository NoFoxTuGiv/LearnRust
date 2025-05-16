#![allow(unused)]

use raylib::prelude::*;

enum SketchState {
    Splash,
    Sketch,
    Ending,
}

fn main() {
    let screen_width = 800;
    let screen_height = 600;

    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("RayLib Test")
        .build();

    rl.set_target_fps(60);
    let mut frame_count = 0;

    let mut current_state = SketchState::Splash;

    while !rl.window_should_close() {


        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        match current_state {
            SketchState::Splash => {
                draw_nofox(&mut d, &screen_width, &screen_height, &frame_count);
                d.draw_text(&frame_count.to_string(), 50, 50, 50, Color::RAYWHITE);
                frame_count += 1;
                if frame_count > 150 { current_state = SketchState::Sketch }
            },
            SketchState::Sketch=> {
            },
            SketchState::Ending=> {},
        }

    }
}

struct Ball {
    pos: Vector2,
    vel: Vector2,
    r: i32,
}

impl Ball {
    fn update() {

    }

    fn draw() {

    }
}

fn handle_splash(d: &mut RaylibDrawHandle, sw: &i32, sh: &i32, fc: &mut i32, cur_state: SketchState) -> SketchState {
    draw_nofox(d, sw, sh, fc);
    d.draw_text(&fc.to_string(), 50, 50, 50, Color::RAYWHITE);
    *fc += 1;
    if *fc > 150 {
        SketchState::Sketch
    } else {
        SketchState::Splash
    }
}

fn handle_sketch(d: &mut RaylibDrawHandle, sw: &i32, sh: &i32, fc: &mut i32, cur_state: SketchState) -> SketchState {
    // Let's start with a bouncing ball.
    SketchState::Sketch
}

fn draw_nofox(d: &mut RaylibDrawHandle, sw: &i32, sh: &i32, fc: &i32) {
    let font_size = 50;
    let text = "NoFoxTuGiv";
    let text_width = d.measure_text(text, font_size);
    let x = (sw - text_width) / 2;
    let y = (sh - font_size) / 2;

    d.draw_text(text, x, y, font_size, Color::RAYWHITE);
}
