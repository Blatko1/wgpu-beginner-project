use crate::model::{Mesh, Model};
use nalgebra::{Rotation3, Translation3, Vector3};
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    pub matrix: [[f32; 4]; 4],
}

impl InstanceRaw {
    pub fn init_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 2,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 3,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 4,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: std::mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 5,
                },
            ],
        }
    }
}

#[derive(Clone)]
pub struct Instance {
    pub position: Vector3<f32>,
    pub rotation: Rotation3<f32>,
}

impl Instance {
    pub fn new(position: Vector3<f32>, rotation: Rotation3<f32>) -> Self {
        Self { position, rotation }
    }

    pub fn to_raw(&self) -> InstanceRaw {
        InstanceRaw {
            matrix: (Translation3::from(self.position).to_homogeneous()
                * self.rotation.matrix().to_homogeneous())
            .into(),
        }
    }
}

pub struct InstanceCollection {
    pub instances: Vec<Instance>,
    pub model: Model,
    pub buffer: wgpu::Buffer,
}

impl InstanceCollection {
    pub fn new(name: &str, model: Model, instances: Vec<Instance>, device: &wgpu::Device) -> Self {
        let instance_raw_vec = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{} vertex buffer", name)),
            contents: bytemuck::cast_slice(&instance_raw_vec),
            usage: wgpu::BufferUsage::VERTEX
        });
        Self {
            instances,
            model,
            buffer,
        }
    }
}
