use crate::camera::Camera;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Debug)]
pub struct MatrixUniform {
    pub proj_view_model_matrix: [[f32; 4]; 4],
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub buffer: wgpu::Buffer,
}

impl MatrixUniform {
    pub fn new(device: &wgpu::Device, camera: &Camera) -> Self {
        let proj_view_model_matrix: [[f32; 4]; 4] = camera.create_view_proj_model_matrix().into();

        let matrix_uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[proj_view_model_matrix]),
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });
        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("uniform_bind_group_layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });
        let matrix_uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("uniform_bind_group"),
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: matrix_uniform_buffer.as_entire_binding(),
            }],
        });
        MatrixUniform {
            proj_view_model_matrix,
            bind_group: matrix_uniform_bind_group,
            bind_group_layout: uniform_bind_group_layout,
            buffer: matrix_uniform_buffer,
        }
    }

    pub fn update_uniform(&mut self, camera: &mut Camera) {
        self.proj_view_model_matrix = camera.create_view_proj_model_matrix().into();
    }
}
