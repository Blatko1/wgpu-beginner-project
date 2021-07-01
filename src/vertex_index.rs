pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3]
}

impl Vertex {
    pub fn init_buffer_layout() -> wgpu::VertexBufferLayout {
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
                    offset: std::mem::sizeof::<[f32;3]>() as wgpu::BufferAddress,
                    shader_location: 1
                }
            ]
        }
    }
}