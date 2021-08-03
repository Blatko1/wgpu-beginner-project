use crate::graphics::Graphics;
use crate::modeling::instance::InstanceRaw;
use crate::modeling::vertex_index::Vertex;
use crate::rendering::graphics::Graphics;

pub struct Object {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
    bind_groups: Vec<wgpu::BindGroup>,
    indices_len: u32,
    instances_len: u32,
}

impl Object {
    /// Function for drawing instanced (or not) objects.
    ///
    /// [`bind_groups`] argument should be arranged so that the 'set = 0' is at `bind_groups[0]` and so on.
    pub fn new(
        graphics: &Graphics,
        vertices: &[Vertex],
        indices: &[u32],
        instance_data: Vec<InstanceRaw>,
        bind_groups: Vec<wgpu::BindGroup>,
    ) -> Self {
        use wgpu::util::DeviceExt;

        let vertex_buffer = graphics
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsage::VERTEX,
            });
        let index_buffer = graphics
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(indices),
                usage: wgpu::BufferUsage::INDEX,
            });
        let instance_buffer =
            graphics
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(&instance_data),
                    usage: wgpu::BufferUsage::VERTEX | wgpu::BufferUsage::COPY_DST,
                });
        let indices_len = indices.len() as u32;
        let instances_len = instance_data.len() as u32;
        Self {
            vertex_buffer,
            index_buffer,
            instance_buffer,
            bind_groups,
            indices_len,
            instances_len,
        }
    }
}

pub trait DrawObject<'a> {
    fn draw_object(&mut self, object: &'a Object);
}

impl<'a> DrawObject<'a> for wgpu::RenderPass<'a> {
    fn draw_object(&mut self, object: &'a Object) {
        self.set_vertex_buffer(0, object.vertex_buffer.slice(..));
        self.set_vertex_buffer(1, object.instance_buffer.slice(..));
        self.set_index_buffer(object.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        for (i, b) in object.bind_groups.iter().enumerate() {
            self.set_bind_group(i as u32, b, &[]);
        }
        self.draw_indexed(0..object.indices_len, 0, 0..object.instances_len);
    }
}
