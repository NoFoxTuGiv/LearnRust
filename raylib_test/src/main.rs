use raylib::prelude::*;

fn main() {
    let screen_width = 800;
    let screen_height = 600;

    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("NoFoxTuGiv")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        let font_size = 50;
        let text = "NoFoxTuGiv";
        let text_width = d.measure_text(text, font_size);
        let x = (screen_width - text_width) / 2;
        let y = (screen_height - font_size) / 2;

        d.draw_text(text, x, y, font_size, Color::WHITE);
    }
}

