#![allow(unused)]

//! A steering behaviors simulation using the `macroquad` graphics library.
//!
//! This program demonstrates a single `Vehicle` agent that can perform
//! several steering behaviors:
//! - **Seek**: Move towards the mouse cursor.
//! - **Flee**: Move away from the mouse cursor.
//! - **Boundary Avoidance**: Stay within the screen bounds.
//!
//! Use the keyboard to toggle behaviors and debug visuals.
//!
//! ## Controls
//! - `S`: Toggle seek behavior.
//! - `F`: Toggle flee behavior.
//! - `D`: Toggle debug view (shows forces and state).
//! - `Esc`: Exit the application.

mod vehicle;

use std::fmt;

use macroquad::prelude::*;
use vehicle::Vehicle;

/// The main entry point for the application.
///
/// Initializes the simulation, creates a `Vehicle`, and enters the main game loop.
/// The loop handles user input, calculates forces, updates the vehicle's state,
/// and draws everything to the screen on each frame.
#[macroquad::main(window_conf)]
async fn main() {
    // Seed the random number generator for vehicle placement.
    rand::srand(miniquad::date::now() as u64);

    // Initialize the simulation state.
    let mut vehicle: Vehicle = Vehicle::new();
    let mut debug: bool = false;
    let mut state = State::Wander;

    let mut target = Vec2::ZERO;

    // Main loop.
    loop {
        clear_background(DARKGRAY);

        // --- FlowField ---

        // --- Force Calulation ---
        // Reset forces from last frame
        let mut seek_force = Vec2::ZERO;
        let mut flee_force = Vec2::ZERO;

        // Calculate steering force based on the current mode.
        match state {
            State::Seek => {
                target = Vec2::new(mouse_position().0, mouse_position().1);
                seek_force = vehicle.seek(&target);
                vehicle.apply_force(seek_force);
            }
            State::Flee => {
                target = Vec2::new(mouse_position().0, mouse_position().1);
                flee_force = vehicle.flee(&target, vehicle.flee_radius);
                vehicle.apply_force(flee_force);
                // If flee_force is ZERO, there aren't nearby threats, wander this frame instead.
                if flee_force == Vec2::ZERO {
                    vehicle.wander();
                }
            }
            State::Wander => {
                target = Vec2::ZERO - Vec2::splat(10.);
                vehicle.wander();
            }
        }

        // Always calculate the boundary avoidance force to keep the vehicle on screen.
        let boundary_force = vehicle.boundary_force(100.);
        vehicle.apply_force(boundary_force);

        // --- Vehicle Logic ---
        vehicle.update();

        // --- Debug and UI ---
        if debug {
            draw_debug(&vehicle, &state, &target, &seek_force, &flee_force, &boundary_force);
        }

        let state_str = format!("Current State: {}", state);
        draw_text(&state_str, 10., screen_height() - 60., 24., GRAY);
        draw_text("Press Esc to Exit.", 10., screen_height() - 30., 24., GRAY);
        draw_text("Press D for Debug", 10., 20., 24., GRAY);
        draw_text("Press S for Seeking", 10., 50., 24., GRAY);
        draw_text("Press F for Fleeing", 10., 80., 24., GRAY);

        // Vehicle Drawing
        vehicle.show();

        // --- Handle Keystrokes ---
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_key_pressed(KeyCode::D) {
            debug = !debug;
            vehicle.toggle_debug();
        }

        if is_key_pressed(KeyCode::S) {
            if state != State::Seek {
                state = State::Seek;
            } else {
                state = State::Wander;
            }
        }

        if is_key_pressed(KeyCode::F) {
            if state != State::Flee {
                state = State::Flee;
            } else {
                state = State::Wander;
            }
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
        window_title: "Vehicles and Steering by NoFoxTuGiv".into(),
        ..Default::default()
    }
}

/// Draws debug information on the screen, including forces and the current state.
///
/// # Arguments
/// * `vehicle` - The `Vehicle` whose forces and state are being debugged.
/// * `state` - The current `State` of the simulation.
/// * `target` - The current target `Vec2` for seek/flee behaviors.
/// * `seek_force` - The `Vec2` representing the calculated seek force.
/// * `flee_force` - The `Vec2` representing the calculated flee force.
/// * `boundary_force` - The `Vec2` representing the calculated boundary avoidance force.
fn draw_debug(
    vehicle: &Vehicle,
    state: &State,
    target: &Vec2,
    seek_force: &Vec2,
    flee_force: &Vec2,
    boundary_force: &Vec2,
) {
    draw_text("DEBUG", screen_width() - 65., 20., 24., BLUE);

    let v_pos = vehicle.position;
    let scale = 5000.;

    // Draw Target
    draw_ellipse(target.x, target.y, 10., 10., 0., GRAY);

    // Draw Seek/Flee vector
    match state {
        State::Seek => {
            draw_line(
                v_pos.x,
                v_pos.y,
                v_pos.x + seek_force.x * scale,
                v_pos.y + seek_force.y * scale,
                2.,
                GREEN,
            );
        }
        State::Flee => {
            draw_line(
                v_pos.x,
                v_pos.y,
                v_pos.x + flee_force.x * scale,
                v_pos.y + flee_force.y * scale,
                2.,
                RED,
            );
        }
        _ => {}
    }

    // Draw boundary vectors in blue
    draw_line(
        v_pos.x,
        v_pos.y,
        v_pos.x + boundary_force.x * scale,
        v_pos.y + boundary_force.y * scale,
        2.,
        BLUE,
    );
}

/// Represents the current steering behavior state of the vehicle.
#[derive(PartialEq)]
enum State {
    Wander,
    Flee,
    Seek,
}

/// Implements 'Display' trait for 'State' enum, allowing it to be printed.
impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            State::Wander => write!(f, "Wandering"),
            State::Flee => write!(f, "Fleeing"),
            State::Seek => write!(f, "Seeking"),
        }
    }
}
