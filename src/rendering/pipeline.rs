use std::num::NonZeroU32;
use std::path::Path;
use crate::rendering::object::Object;
use crate::rendering::graphics::Graphics;

pub struct Pipeline {
    pub render_pipeline: wgpu::RenderPipeline,
    pub objects: Vec<Object>,
}

impl Pipeline {
    pub fn new<P: AsRef<Path>>(
        label: &str,
        graphics: &Graphics,
        vertex_path: P,
        fragment_path: P,
        depth_format: Option<wgpu::TextureFormat>,
        bind_group_layouts: &[&wgpu::BindGroupLayout],
        vertex_buffer_layouts: &[wgpu::VertexBufferLayout],
    ) -> Self {
        let vertex_shader = graphics
            .device
            .create_shader_module(&wgpu::ShaderModuleDescriptor {
                label: Some(&format!("{} vertex shader", label)),
                source: wgpu::util::make_spirv(include_bytes!(vertex_path)),
                flags: wgpu::ShaderFlags::all(),
            });
        let fragment_shader = graphics
            .device
            .create_shader_module(&wgpu::ShaderModuleDescriptor {
                label: Some(&format!("{} fragment shader", label)),
                source: wgpu::util::make_spirv(include_bytes!(fragment_path)),
                flags: wgpu::ShaderFlags::empty(),
            });
        let layout = graphics
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some(&format!("{} render pipeline layout", label)),
                bind_group_layouts,
                push_constant_ranges: &[],
            });
        let render_pipeline =
            graphics
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some(&format!("{} render pipeline", label)),
                    layout: Some(&layout),
                    vertex: wgpu::VertexState {
                        module: &vertex_shader,
                        entry_point: "main",
                        buffers: vertex_buffer_layouts,
                    },
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: Some(wgpu::Face::Back),
                        clamp_depth: false,
                        polygon_mode: wgpu::PolygonMode::Fill,
                        conservative: false,
                    },
                    depth_stencil: depth_format.map(|f| wgpu::DepthStencilState {
                        format: f,
                        depth_write_enabled: true,
                        depth_compare: wgpu::CompareFunction::Less,
                        stencil: wgpu::StencilState::default(),
                        bias: wgpu::DepthBiasState::default(),
                    }),
                    multisample: wgpu::MultisampleState {
                        count: 1,
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &fragment_shader,
                        entry_point: "main",
                        targets: &[wgpu::ColorTargetState {
                            format: graphics.sc_desc.format,
                            blend: Some(wgpu::BlendState::REPLACE),
                            write_mask: wgpu::ColorWrite::ALL,
                        }],
                    }),
                });
        Self { render_pipeline }
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }
}
