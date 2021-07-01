pub trait VertexLayout {
    fn init_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a>;
}
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3]
}

impl VertexLayout for Vertex {
    fn init_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout{
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute{
                    format: wgpu::VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0
                },
                wgpu::VertexAttribute{
                    format: wgpu::VertexFormat::Float32x3,
                    offset: std::mem::size_of::<[f32;3]>() as wgpu::BufferAddress,
                    shader_location: 1
                }
            ]
        }
    }
}