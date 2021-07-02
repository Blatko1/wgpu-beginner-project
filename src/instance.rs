use wgpu::util::DeviceExt;
use nalgebra::{Vector3, Rotation3};
use crate::model::Model;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    pub matrix: [[f32; 4]; 4]
}

impl InstanceRaw {
    pub fn init_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout{
            array_stride: std::mem::size_of::<[[f32; 4]; 4]>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute{
                    format: wgpu::VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 2
                },
                wgpu::VertexAttribute{
                    format: wgpu::VertexFormat::Float32x4,
                    offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 3
                },
                wgpu::VertexAttribute{
                    format: wgpu::VertexFormat::Float32x4,
                    offset: std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 4
                },
                wgpu::VertexAttribute{
                    format: wgpu::VertexFormat::Float32x4,
                    offset: std::mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 5
                },
            ]
        }
    }
}

pub struct Instance {
    pub model: Model,
    pub position: Vector3<f32>,
    pub rotation: Rotation3<f32>,
}

impl Instance {

    fn new(position: Vector3<f32>, rotation: Rotation3<f32>, device: &wgpu::Device) -> Self {
        Self {
            position,
            rotation,
        }
    }

    fn to_raw(&self) -> InstanceRaw {

    }
}

pub struct InterfaceBuffer {
    data: Vec<InstanceRaw>,
}

impl InterfaceBuffer {
    pub fn new_buffer(&self, name: &str, device: &wgpu::Device) -> wgpu::Buffer {
        let data = self.data.iter().as_slice();
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{} vertex buffer", name)),
            contents: bytemuck::cast_slice(&data),
            usage: wgpu::BufferUsage::UNIFORM
        })
    }
}