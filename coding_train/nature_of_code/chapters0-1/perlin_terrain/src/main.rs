use macroquad::prelude::*;
use macroquad::models::{Mesh, Vertex};
use macroquad::models::draw_mesh;

#[macroquad::main("Custom Mesh")]
async fn main() {
    // define vertices (x, y, z, u, v, color)
    let verts = vec![
        Vertex::new(100., 100., 0., 0., 0., PURPLE),
        Vertex::new(200., 100., 0., 0., 0., PURPLE),
        Vertex::new(100., 200., 0., 0., 0., PURPLE),
        Vertex::new(200., 200., 0., 0., 0., PURPLE),
    ];

    // these indices pull from `verts` to make one square
    // 0----1
    // |   /|
    // |  / |
    // | /  |
    // 2----3
    // 0 > 1 > 2, then 1 > 3 > 2
    let inds  = vec![0, 1, 2, 1, 3, 2];

    // no texture, just solid color
    let mesh  = Mesh { vertices: verts, indices: inds, texture: None };

    loop {
        clear_background(BLACK);
        draw_mesh(&mesh);
        next_frame().await;
    }
}

