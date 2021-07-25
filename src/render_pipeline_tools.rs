pub fn new_render_pipeline(
    name: &str,
    device: &wgpu::Device,
    group_layouts: &[&wgpu::BindGroupLayout],
    vert_shader: &wgpu::ShaderModule,
    frag_shader: &wgpu::ShaderModule,
    color_format: wgpu::TextureFormat,
    depth_format: wgpu::TextureFormat,
    vert_layout: &[wgpu::VertexBufferLayout],
) -> wgpu::RenderPipeline {
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some(&format!("{} pipeline layout.", name)),
        bind_group_layouts: group_layouts,
        push_constant_ranges: &[],
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(&format!("{} render pipeline.", name)),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: vert_shader,
            entry_point: "main",
            buffers: vert_layout,
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
        depth_stencil: Some(wgpu::DepthStencilState {
            format: depth_format,
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
            module: frag_shader,
            entry_point: "main",
            targets: &[wgpu::ColorTargetState {
                format: color_format,
                blend: Some(wgpu::BlendState {
                    color: wgpu::BlendComponent::REPLACE,
                    alpha: wgpu::BlendComponent::REPLACE,
                }),
                write_mask: wgpu::ColorWrite::ALL,
            }],
        }),
    })
}
