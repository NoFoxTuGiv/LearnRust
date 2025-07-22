use std::f32::consts::TAU;

use macroquad::prelude::*;
use noise::{Fbm, NoiseFn, Perlin};

/// Represents a 2D grid of vectors; a "flow field" that
/// can guide the movement of agents (like our `Vehicle`s).
///
/// Each cell in this grid holds a `Vec2` which indicates a direction.
pub struct FlowField {
    /// The size of each cell in the grid.
    resolution: f32,
    /// The number of columns in the grid.
    cols: usize,
    /// The number of rows in the grid.
    rows: usize,
    /// The 2D vector field itself, stored as a vector of vectors of `Vec2`.
    field: Vec<Vec<Vec2>>,
    /// The Z-axis for noise scrolling
    zoff: f64,
    /// The Z-axis scrolling toggle
    pub scroll_z: bool,
}

impl FlowField {
    const SAMPLE_RATE: f64 = 0.03;

    /// Creates a new `FlowField` with a specified cell `resolution`.
    ///
    /// The field is filled with direction vectors generated from Perlin noise,
    /// giving it a natural, flowing appearance.
    ///
    /// # Arguments
    /// * `resolution` - The desired pixel size for each cell in the flow field.
    /// * `noise` - A reference to an `Fbm<Perlin>` noise generator, used to create the field's "flow."
    ///
    /// # Returns
    /// A new `FlowField` instance.
    pub fn new(resolution: f32, noise: &Fbm<Perlin>) -> FlowField {
        // Calculate the number of columns and rows based on screen size and resolution.
        let cols = (screen_width() / resolution).floor() as usize;
        let rows = (screen_height() / resolution).floor() as usize;
        // Pre-allocate space for the field to be efficient.
        let mut field: Vec<Vec<Vec2>> = Vec::with_capacity(cols);

        let mut xoff: f64 = 0.; // X-offset for noise sampling.
        for _ in 0..cols {
            let mut yoff: f64 = 0.; // Y-offset for noise sampling.
            let mut col: Vec<Vec2> = Vec::with_capacity(rows);
            for _ in 0..rows {
                let angle = map_range(noise.get([xoff, yoff]) as f32, 0., 1., 0., TAU);
                let cell = Vec2::from_angle(angle);
                col.push(cell);
                yoff += Self::SAMPLE_RATE;
            }
            xoff += Self::SAMPLE_RATE;
            field.push(col);
        }

        FlowField {
            resolution,
            cols,
            rows,
            field,
            zoff: 0.,
            scroll_z: false,
        }
    }

    /// Draws the flow field vectors on the screen as arrows for debugging.
    ///
    /// Each vector is drawn as a line with a small triangle arrowhead at its end,
    /// indicating its direction.
    pub fn show(&self) {
        for x in 0..self.cols {
            for y in 0..self.rows {
                let cell_vector = self.field[x][y];

                // Calculate the center of the cell
                let c_center_x = (x as f32 * self.resolution) + (self.resolution * 0.5);
                let c_center_y = (y as f32 * self.resolution) + (self.resolution * 0.5);
                let cell_center = Vec2::new(c_center_x, c_center_y);

                let direction = cell_vector.normalize_or_zero();
                let len = self.resolution * 0.8;
                let half_scl_vec = direction * (len * 0.5);

                let start_point = cell_center - half_scl_vec;
                let end_point = cell_center + half_scl_vec;

                draw_line(
                    start_point.x,
                    start_point.y,
                    end_point.x,
                    end_point.y,
                    2.,
                    DARKBLUE,
                );
            }
        }
    }

    /// Regenerates the entire flow field using a new noise instance.
    ///
    /// This is called when the user presses the SpaceBar to get a new, randomized field.
    ///
    /// # Arguments
    /// * `noise` - A reference to a new `Fbm<Perlin>` noise generator.
    pub fn generate_field(&mut self, noise: &Fbm<Perlin>) {
        let mut xoff: f64 = 0.;
        for x in 0..self.cols {
            let mut yoff: f64 = 0.;
            for y in 0..self.rows {
                let angle = map_range(noise.get([xoff, yoff, self.zoff]) as f32, 0., 1., 0., TAU);
                let cell = Vec2::from_angle(angle);
                self.field[x][y] = cell;
                yoff += Self::SAMPLE_RATE;
            }
            xoff += Self::SAMPLE_RATE;
        }
    }

    pub fn update_field(&mut self, noise: &Fbm<Perlin>) {
        let mut xoff: f64 = 0.;
        for x in 0..self.cols {
            let mut yoff: f64 = 0.;
            for y in 0..self.rows {
                let angle = map_range(noise.get([xoff, yoff, self.zoff]) as f32, 0., 1., 0., TAU);
                let cell = Vec2::from_angle(angle);
                self.field[x][y] = cell;
                yoff += Self::SAMPLE_RATE;
            }
            xoff += Self::SAMPLE_RATE;
        }
        if self.scroll_z {
            self.zoff += 0.0005;
        }
    }

    /// Looks up the direction vector in the flow field at a given screen `position`.
    ///
    /// This is how a `Vehicle` figures out which way the flow field is telling it to go.
    ///
    /// # Arguments
    /// * `position` - The `Vec2` representing the screen coordinates to look up.
    ///
    /// # Returns
    /// The `Vec2` direction vector from the flow field at that position.
    pub fn lookup(&self, position: &Vec2) -> Vec2 {
        let col = constrain(
            (position.x / self.resolution).floor() as usize,
            0,
            self.cols - 1,
        );
        let row = constrain(
            (position.y / self.resolution).floor() as usize,
            0,
            self.rows - 1,
        );
        self.field[col][row]
    }

    pub fn get_status(&self) -> String {
        if self.scroll_z {
            "Dynamic".into()
        } else {
            "Static".into()
        }
    }
}

// --- Helper Functions ---

/// Constrains a given number `n` to be within a specified `min` and `max` range.
///
/// If `n` is less than `min`, it returns `min`. If `n` is greater than `max`, it returns `max`.
/// Otherwise, it returns `n` itself.
///
/// # Arguments
/// * `n` - The number to constrain.
/// * `min` - The minimum allowed value.
/// * `max` - The maximum allowed value.
///
/// # Returns
/// The constrained `usize` value.
fn constrain(n: usize, min: usize, max: usize) -> usize {
    n.max(min).min(max)
}

/// Remaps a number from one numerical range to another.
///
/// This is super handy for converting values, like a noise value (0 to 1)
/// into an angle (0 to $2\pi$).
///
/// # Arguments
/// * `value` - The number you want to convert.
/// * `in_min` - The lowest value in the current range.
/// * `in_max` - The highest value in the current range.
/// * `out_min` - The lowest value in the target range.
/// * `out_max` - The highest value in the target range.
///
/// # Type Parameters
/// * `T` - This function is generic and works with any numeric type that supports
///   basic math operations (addition, subtraction, multiplication, division) and can be copied.
///   Examples include `f32`, `f64`, `i32`, `u64`, etc.
///
/// # Returns
/// The `value` remapped to the new range.
fn map_range<T>(value: T, in_min: T, in_max: T, out_min: T, out_max: T) -> T
where
    T: std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + Copy,
{
    out_min + (out_max - out_min) * ((value - in_min) / (in_max - in_min))
}
