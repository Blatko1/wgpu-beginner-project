use crate::modeling::model::Mesh;
use crate::modeling::vertex_index::Vertex;
use rand::Rng;

/*#[allow(dead_code)]
pub fn flat_terrain(name: &str, width: u32, height: u32, device: &wgpu::Device) -> Mesh {
    let mut vertices: Vec<Vertex> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    for m in 0..height {
        for n in 0..width {
            let mut rng = rand::thread_rng();
            let vert = Vertex {
                position: [n as f32, rng.gen(), m as f32],
                tex_cords: [
                    1.0 / (width as f32) * (n as f32),
                    1.0 / (height as f32) * (m as f32),
                ],
                color: [0.5, 0.5, 0.5],
                normal: [0.0, 0.0, 0.0],
            };
            vertices.push(vert);
        }
    }

    for m in 0..height - 1 {
        for n in 0..width - 1 {
            let top_left = n + (m * width);
            let top_right = top_left + 1;
            let bottom_left = ((m + 1) * width) + n;
            let bottom_right = bottom_left + 1;
            indices.push(top_left);
            indices.push(bottom_left);
            indices.push(top_right);
            indices.push(top_right);
            indices.push(bottom_left);
            indices.push(bottom_right);
        }
    }

    Mesh::custom_mesh(name, device, vertices.as_slice(), indices.as_slice(), 0)
}*/
