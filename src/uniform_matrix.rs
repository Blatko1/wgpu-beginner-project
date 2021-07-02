use crate::camera::Camera;
use nalgebra::Matrix4;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MatrixUniform {
    proj_view_model_matrix: [[f32; 4]; 4],
}

impl MatrixUniform {
    pub fn new() -> Self {
        MatrixUniform {
            proj_view_model_matrix: Matrix4::identity().into(),
        }
    }

    pub fn update_uniform(&mut self, camera: &mut Camera) {
        self.proj_view_model_matrix = camera.create_view_proj_model_matrix().into();
    }
}
