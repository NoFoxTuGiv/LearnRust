use macroquad::prelude::*;
use macroquad::models::{ draw_mesh, Mesh, Vertex};
use noise::{Fbm, NoiseFn, Perlin};
use miniquad::date;

const WIDTH:        f32 = 2000.;
const HEIGHT:       f32 = 800.;
const SCL:          f32 = 30.;
const FLY_SPEED:    f32 = 0.006;
const HEIGHT_AMPL:  f32 = 65.0;
const COLS:         usize = (WIDTH / SCL) as usize;
const ROWS:         usize = (HEIGHT / SCL) as usize;
const CHUNK: usize = 16;

#[macroquad::main(window_conf)]
async fn main() {
    let mut flying: f32 = 0.;
    rand::srand(date::now() as u64);
    let seed = rand::gen_range(0,u32::MAX);
    let noise = Fbm::<Perlin>::new(seed);

    let purple = Color::new(0.40, 0.05, 0.60, 1.0);
    let teal = Color::new(0.00, 0.80, 0.75, 1.0);
    let max_dis = (vec2(COLS as f32 * SCL * 0.5, ROWS as f32 * SCL * 0.5)).length();

    let world_center = vec3(
        0.,
        0.,
        0.,
    );

    let camera = Camera3D {
        position: world_center + vec3(25., 400., -400.),
        target: world_center,
        up: vec3(0., 1., 0.),
        fovy: 75f32.to_radians(),
        ..Default::default()
    };

    loop {
        flying += FLY_SPEED;
        clear_background(BLACK);
        set_camera(&camera);

        for base_j in (0..ROWS).step_by(CHUNK) {
            for base_i in (0..COLS).step_by(CHUNK) {
                let patch_cols = (COLS - base_i).min(CHUNK + 1);
                let patch_rows = (ROWS - base_j).min(CHUNK + 1);

                let mut verts = Vec::with_capacity(patch_cols * patch_rows);

                let mut y_off = flying as f64 + (base_j as f32 * 0.1) as f64;
                for local_j in 0..patch_rows {
                    let mut x_off = (base_i as f32 * 0.1) as f64;
                    for local_i in 0..patch_cols {
                        let h = noise.get([x_off, y_off]) as f32;
                        let y = map_range(h, 0., 1., -HEIGHT_AMPL, HEIGHT_AMPL);
                        let x = ((base_i + local_i) as f32 * SCL) - (COLS as f32 * SCL * 0.5);
                        let z = ((base_j + local_j) as f32 * SCL) - (ROWS as f32 * SCL * 0.5);
                        let center_dis = vec2(x, 0.0).length();
                        let t = (1.0 - (center_dis / max_dis)).powf(2.8).min(1.0);
                        let fill_color = Color::new(
                            lerp(purple.r, teal.r, t),
                            lerp(purple.g, teal.g, t),
                            lerp(purple.b, teal.b, t),
                            0.9,
                        );
                        verts.push(Vertex::new2(vec3(x, y, z), vec2(0., 0.), fill_color));
                        x_off += 0.1;
                    }
                    y_off += 0.1;
                }

                let mut inds = Vec::with_capacity((patch_cols - 1) * (patch_rows - 1) * 6);
                for lj in 0..patch_rows - 1 {
                    for li in 0..patch_cols - 1 {
                        let tl = (lj * patch_cols + li) as u16;
                        let tr = tl + 1;
                        let bl = ((lj+1) * patch_cols + li) as u16;
                        let br = bl + 1;

                        inds.extend_from_slice(&[tl, bl, tr, tr, bl, br]);
                    }
                }

                let mesh = Mesh { vertices: verts, indices: inds, texture: None };
                draw_mesh(&mesh);
            }
        }
        set_default_camera();

        next_frame().await;
    }

}

fn map_range(
    value: f32,
    in_min: f32,
    in_max: f32,
    out_min: f32,
    out_max: f32,
) -> f32 {
    out_min + (value - in_min) / (in_max - in_min) * (out_max - out_min)
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Perlin Terrain".to_owned(),
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        high_dpi: true,
        ..Default::default()
    }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 { a + ( b - a) * t }
