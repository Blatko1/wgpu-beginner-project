use crate::vertex_index::Vertex;
use std::ops::Range;
use wgpu::util::DeviceExt;

pub struct Material {
    // TODO
}

pub struct Mesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_length: u32,
}

impl Mesh {
    fn draw_mesh<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>) {
        self.draw_mesh_instanced(pass, 0..1);
    }

    fn draw_mesh_instanced<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>, instances: Range<u32>, ) {
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        pass.draw_indexed(0..self.index_length, 0, instances);
    }

    pub fn custom_mesh(name: &str, device: &wgpu::Device, vertices: &[Vertex], indices: &[u16]) -> Self {
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
    mesh: Vec<Mesh>,
    model_matrix: [[f32; 4]; 4],
}

impl Model {
    fn draw_model<'a>(&'a mut self, pass: &mut wgpu::RenderPass<'a>) {
        self.draw_model_instanced(pass, 0..1);
    }

    fn draw_model_instanced<'a>(&'a mut self, pass: &mut wgpu::RenderPass<'a>, instances: Range<u32>) {
        for m in &mut self.mesh {
            m.draw_mesh_instanced(pass, instances.clone());
        }
    }
}
