use macroquad::{
    models::{draw_mesh, Mesh, Vertex},
    prelude::*,
};
use noise::{ Fbm, NoiseFn, Perlin};

const WIDTH: f32 = 400.;
const HEIGHT: f32 = 300.;
const SCL: f32 = 2.;

const COLS: usize = (WIDTH / SCL) as usize;
const ROWS: usize = (HEIGHT / SCL) as usize;
const CHUNK: usize = 16;

const CLOUD_ALT: f32 = 200.0;
const SPEED: f32 = 0.006;
const NOISE_SCALE: f64 = 0.10;

struct Chunk {
    mesh: Mesh,
}

#[macroquad::main(window_conf)]
async fn main() {
    rand::srand(miniquad::date::now() as u64);
    let seed = rand::gen_range(0, u32::MAX);
    let noise = Fbm::<Perlin>::new(seed);


    let mut chunks: Vec<Chunk> = Vec::new();
    for base_j in (0..ROWS).step_by(CHUNK) {
        for base_i in (0..COLS).step_by(CHUNK) {
            let patch_cols = (COLS - base_i).min(CHUNK + 1);
            let patch_rows = (ROWS - base_j).min(CHUNK + 1);

            let mut verts = Vec::with_capacity(patch_cols * patch_rows);
            for j in 0..patch_rows {
                for i in 0..patch_cols {
                    let x = (base_i + i) as f32 * SCL - COLS as f32 * SCL * 0.5;
                    let z = (base_j + j) as f32 * SCL - ROWS as f32 * SCL * 0.5;
                    verts.push(Vertex::new2(vec3(x, CLOUD_ALT, z),
                                            vec2(0., 0.),
                                            Color::from_rgba(255, 255, 255, 0)));
                }
            }

            let mut inds = Vec::with_capacity((patch_cols - 1) * (patch_rows - 1) * 6);
            for j in 0..patch_rows - 1 {
                for i in 0..patch_cols - 1 {
                    let tl = (j * patch_cols + i) as u16;
                    let tr = tl + 1;
                    let bl = ((j + 1) * patch_cols + i) as u16;
                    let br = bl + 1;
                    inds.extend_from_slice(&[tl, bl, tr, tr, bl, br]);
                }
            }

            chunks.push(Chunk {
                mesh: Mesh{ vertices: verts, indices: inds, texture: None },
            });
        }
    }

    let camera = Camera3D {
        position: vec3(0., CLOUD_ALT + 600., 0.),
        target:   vec3(0., CLOUD_ALT, 0.),
        up:       vec3(0., 0., 1.),
        projection: Projection::Orthographics,
        fovy: WIDTH / 5.,
        ..Default::default()
    };

    let mut t = 0.;
    loop{
        t += SPEED;

        for chunk in &mut chunks {
            for v in &mut chunk.mesh.vertices {
                let x_off = v.position.x as f64 * NOISE_SCALE;
                let y_off = v.position.z as f64 * NOISE_SCALE + t as f64;

                let n = (noise.get([x_off, y_off]) as f32 * 0.5) + 0.5;
                v.color[3] = ((n.powf(2.2)) * 255.) as u8;
            }
        }

        clear_background(sky_blue());
        set_camera(&camera);
        for chunk in &chunks { draw_mesh(&chunk.mesh); }
        set_default_camera();
        
        //draw_fps();

        next_frame().await;
    }
}

fn sky_blue() -> Color { Color::new(0.45, 0.73, 0.92, 1.0) }

fn window_conf() -> Conf {
    Conf {
        window_title: "Perlin Cloud".to_owned(),
        window_width: 400,
        window_height: 300,
        window_resizable: false,
        high_dpi: true,
        ..Default::default()
    }
}
