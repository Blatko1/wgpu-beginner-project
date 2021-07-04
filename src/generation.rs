use crate::model::Mesh;

pub fn random_terrain(name: &str, device: &wgpu::Device) -> Mesh {
    Mesh::custom_mesh(name, device, )
}