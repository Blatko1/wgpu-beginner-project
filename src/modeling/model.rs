use crate::vertex_index::Vertex;
use std::ops::Range;
use wgpu::util::DeviceExt;

#[allow(dead_code)]
pub struct Material {
    // TODO
}

pub struct Mesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_length: u32,
}

impl Mesh {
    #[allow(dead_code)]
    pub fn custom_mesh(
        name: &str,
        device: &wgpu::Device,
        vertices: &[Vertex],
        indices: &[u16],
    ) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{} Vertex Buffer", name)),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsage::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{} Index Buffer", name)),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsage::INDEX,
        });
        let index_length = indices.len() as u32;
        Self {
            vertex_buffer,
            index_buffer,
            index_length,
        }
    }
}

pub struct Model {
    pub mesh: Vec<Mesh>,
}

pub trait DrawModel<'a> {
    fn draw_mesh(&mut self, model: &'a Model, instance_buffer: &'a wgpu::Buffer);
    fn draw_mesh_instanced(
        &mut self,
        model: &'a Model,
        instance_buffer: &'a wgpu::Buffer,
        instances: Range<u32>,
    );
}

impl<'a> DrawModel<'a> for wgpu::RenderPass<'a> {
    fn draw_mesh(&mut self, model: &'a Model, instance_buffer: &'a wgpu::Buffer) {
        self.draw_mesh_instanced(model, instance_buffer, 0..1);
    }

    fn draw_mesh_instanced(
        &mut self,
        model: &'a Model,
        instance_buffer: &'a wgpu::Buffer,
        instances: Range<u32>,
    ) {
        for m in &model.mesh {
            self.set_vertex_buffer(0, m.vertex_buffer.slice(..));
            self.set_vertex_buffer(1, instance_buffer.slice(..));
            self.set_index_buffer(m.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            self.draw_indexed(0..m.index_length, 0, instances.clone());
        }
    }
}
