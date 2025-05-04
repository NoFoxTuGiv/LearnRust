#![allow(unused)]

use macroquad::prelude::*;

#[macroquad::main("3D Bounce")]
async fn main() {
    loop{
        clear_background(LIGHTGRAY);

        set_camera(&Camera3D {
            position: vec3(-20., 15., 0.0),
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        });

        // This is not visible
        draw_cube_wires(vec3(screen_width() / 2., screen_height() / 2., 6.), vec3(2., 2., 2.), WHITE);

        // These are visible
        draw_cube_wires(vec3(0., 1., -6.), vec3(2., 2., 2.), DARKGREEN);
        draw_cube_wires(vec3(0., 1., 6.), vec3(2., 2., 2.), DARKBLUE);
        draw_cube_wires(vec3(2., 1., 2.), vec3(2., 2., 2.), YELLOW);

        draw_cube(vec3(2., 0., -2.), vec3(0.4, 0.4, 0.4), None, BLACK);
        
        set_default_camera();

        next_frame().await;
    }
}
