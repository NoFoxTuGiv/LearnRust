//! A flowfield behavior simulation using the `macroquad` graphics library.
//!
//! This program demonstrates `Vehicle` agents that follow a flow-field.
//! The flow field itself is generated using Perlin noise.
//!
//! Use the keyboard to interact with the simulation.
//!
//! ## Controls
//! - `D`: Toggle debug view (shows the flow field vectors).
//! - `SpaceBar`: Generate a new, random flow field.
//! - `Esc`: Exit the application.

mod flowfield;
mod vehicle;

use flowfield::FlowField;
use vehicle::Vehicle;
use macroquad::{prelude::*, rand::gen_range};
use noise::{ Fbm, Perlin };

/// The main entry point for the application.
///
/// Initializes the simulation, sets up a collection of `Vehicle` agents,
/// and creates the initial `FlowField`. The main loop then handles
/// user input, updates vehicle behaviors based on the flow field,
/// and renders everything to the screen.
#[macroquad::main(window_conf)]
async fn main() {
    const VEHICLE_CNT: usize = 25;
    const FF_RESOLUTION: f32 = 20.;
    // Seed the random number generator for vehicle placement.
    rand::srand(miniquad::date::now() as u64);

    // Initialize the simulation state.
    let mut vehicles: Vec<Vehicle> = std::iter::repeat_with(Vehicle::new)
        .take(VEHICLE_CNT)
        .collect();

    // Set up the initial noise for the flow field generation.
    let mut r = gen_range(0, u32::MAX);
    let mut noise = Fbm::<Perlin>::new(r);
    // Create the flow field with a specified resolution.
    let mut field: FlowField = FlowField::new(FF_RESOLUTION, &noise);

    // Flag to toggle debug visualization
    let mut debug: bool = false;

    // Main loop.
    loop {
        clear_background(DARKGRAY);

        // --- Vehicle Logic ---
        for vehicle in &mut vehicles {
            vehicle.follow(&field);
            vehicle.update();
        }

        // --- Debug and UI ---
        if debug {
            draw_debug(&field);
        }

        // Control instructions
        draw_text("Press Esc to Exit.", 10., screen_height() - 30., 24., GRAY);
        draw_text("Press D for Debug", 10., 20., 24., GRAY);
        draw_text("Press SpaceBar to generate a new field.", 10., 50., 24., GRAY);

        // Vehicle Drawing
        for vehicle in &vehicles {
            vehicle.show();
        }

        // --- Handle Keystrokes ---
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_key_pressed(KeyCode::Space) {
            r = miniquad::date::now() as u32;
            noise = Fbm::<Perlin>::new(r);
            field.generate_field(&noise);
        }

        if is_key_pressed(KeyCode::D) {
            debug = !debug;
        }

        next_frame().await;
    }
}

/// Provides the initial configuration for the application window.
///
/// # Returns
/// A `Conf` struct with custom settings.
fn window_conf() -> Conf {
    Conf {
        window_title: "FlowFields by NoFoxTuGiv".into(),
        ..Default::default()
    }
}

/// Draws debug information on the screen.
///
/// # Arguments
/// * `field` - The `FlowField` to display.
fn draw_debug(
    field: &FlowField,
) {
    // Draw FlowField
    field.show();
}
