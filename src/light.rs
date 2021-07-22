use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]

struct Light {
    position: [f32; 3],
    // Due to uniforms requiring 16 byte (4 float) spacing, we need to use a padding field here
    _padding: u32,
    color: [f32; 3],
}

/*impl Light {
    fn new_light_buffer(&self, name: &str, device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{} Light Buffer", name)),
            contents: bytemuck::cast_slice(&[self]),
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST
        })
    }
}*/
