use crate::cube::CubeType;
use nalgebra::{Matrix3, Rotation3, Translation3};

pub struct Quad {
    pub position: Translation3<f32>,
    pub rotation: Rotation3<f32>,
    pub offset: (f32, f32),
    pub direction: QuadDirection
}

impl Quad {
    pub fn new(
        position: [f32; 3],
        rotation: Rotation3<f32>,
        offset: (f32, f32),
        direction: QuadDirection,
    ) -> Self {
        Quad {
            position: Translation3::new(position[0], position[1], position[2]),
            rotation,
            offset,
            direction
        }
    }
    pub fn to_raw(&self) -> QuadRaw {
        let matrix: [[f32; 4]; 4] =
            (self.position.to_homogeneous() * self.rotation.matrix().to_homogeneous()).into();
        let n_matrix: [[f32; 3]; 3] = Matrix3::from(self.rotation).into();
        let offset_x_y = match self.direction {
            QuadDirection::SIDE => (0., 0.),
            QuadDirection::DOWN => (0., 1.),
            QuadDirection::UP => (1., 0.)
        };
        let x_offset: f32 = 1.;
        let y_offset: f32 = 3.;
        let offset = [x_offset, y_offset];
        QuadRaw {
            matrix,
            n_matrix,
            offset,
            texture_rows: 2.
        }
    }
}

pub enum QuadDirection {
    UP = 2,
    DOWN = 1,
    SIDE = 0
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct QuadRaw {
    pub matrix: [[f32; 4]; 4],
    pub n_matrix: [[f32; 3]; 3],
    pub offset: [f32; 2],
    pub texture_rows: f32,          // Every map must have same width and height with same amount of rows and columns.
}

impl QuadRaw {
    pub fn init_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<QuadRaw>() as wgpu::BufferAddress,
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
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    offset: std::mem::size_of::<[f32; 26]>() as wgpu::BufferAddress,
                    shader_location: 10,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32,
                    offset: std::mem::size_of::<[f32; 27]>() as wgpu::BufferAddress,
                    shader_location: 11,
                },
            ],
        }
    }
}
