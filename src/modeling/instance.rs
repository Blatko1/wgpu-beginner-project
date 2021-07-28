use crate::modeling::model::Model;
use nalgebra::{Matrix3, Rotation3, Translation3, Vector3};
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    pub matrix: [[f32; 4]; 4],
    pub n_matrix: [[f32; 3]; 3],
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
                    shader_location: 3,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 4,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 5,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: std::mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 6,
                },
                // 3x3 Matrix for normals.
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: std::mem::size_of::<[f32; 16]>() as wgpu::BufferAddress,
                    shader_location: 7,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: std::mem::size_of::<[f32; 19]>() as wgpu::BufferAddress,
                    shader_location: 8,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: std::mem::size_of::<[f32; 22]>() as wgpu::BufferAddress,
                    shader_location: 9,
                },
            ],
        }
    }
}

#[derive(Clone)]
pub struct Instance {
    pub translation: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub radius: Vector3<f32>,
}

impl Instance {
    pub fn new(translation: Vector3<f32>, rotation: Vector3<f32>, radius: Vector3<f32>) -> Self {
        Self {
            translation,
            rotation,
            radius,
        }
    }

    pub fn to_raw(&self) -> InstanceRaw {
        let radius_from_center =
            Translation3::new(self.radius.x, self.radius.y, self.radius.z).to_homogeneous();
        let rot = Rotation3::new(self.rotation);
        let translation =
            Translation3::new(self.translation.x, self.translation.y, self.translation.z)
                .to_homogeneous();
        let n_matrix = Matrix3::from(rot);
        InstanceRaw {
            matrix: (translation * rot.matrix().to_homogeneous() * radius_from_center).into(),
            n_matrix: n_matrix.into(),
        }
    }

    #[allow(dead_code)]
    pub fn translate(&mut self, add_translation: Vector3<f32>) {
        self.translation += add_translation;
    }

    #[allow(dead_code)]
    pub fn rotate(&mut self, add_rotation: Vector3<f32>) {
        self.rotation += add_rotation
    }
}

pub struct ModelRenderInfo {
    pub instances: Vec<Instance>,
    pub model: Model,
    pub instance_buffer: wgpu::Buffer,
}

impl ModelRenderInfo {
    pub fn new(name: &str, model: Model, instances: Vec<Instance>, device: &wgpu::Device) -> Self {
        let instance_raw_vec = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{} vertex buffer", name)),
            contents: bytemuck::cast_slice(&instance_raw_vec),
            usage: wgpu::BufferUsage::VERTEX | wgpu::BufferUsage::COPY_DST,
        });
        Self {
            instances,
            model,
            instance_buffer,
        }
    }
}
