use crate::texture::Texture;
use crate::rendering::graphics::Graphics;
use crate::rendering::uniforms;
use crate::rendering::pipeline::Pipeline;
use crate::rendering::object::DrawObject;
use crate::modeling::vertex_index::{Vertex, VertexLayout};
use crate::modeling::instance::InstanceRaw;

pub const SAMPLED_TEXTURES_COUNT: u32 = 2;

pub struct Renderer {
    pipelines: Vec<Pipeline>,
    depth_texture: Texture,
}

impl Renderer {
    pub fn new(graphics: &Graphics) -> Self {
        let depth_texture =
            Texture::create_depth_texture(&graphics.device, &graphics.sc_desc, "depth texture");
        Self {
            pipelines: Vec::new(),
            depth_texture,
        }
    }

    pub fn render(&self, graphics: &Graphics) -> Result<(), wgpu::SwapChainError> {
        let frame = graphics.swap_chain.get_current_frame()?.output;
        let mut encoder = graphics
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("render command encoder"),
            });
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("main render pass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &frame.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: true,
                },
            }],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth_texture.view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: true,
                }),
                stencil_ops: None,
            }),
        });

        for p in self.pipelines {
            render_pass.set_pipeline(&p.render_pipeline);
            for o in p.objects {
                use crate::object::DrawObject;
                render_pass.draw_object(&o);
            }
        }

        drop(render_pass);

        Ok(())
    }

    fn add_primary_pipeline(&mut self, graphics: &Graphics) {
        let shader_dir = std::path::Path::new(env!("OUT_DIR")).join("src/shaders");
        let v_path = shader_dir.join("shader.vert");
        let f_path = shader_dir.join("shader.frag");

        let bind_group_layout_desc = vec![
            uniforms::MATRIX_UNIFORM_LAYOUT_DESC,
            uniforms::SAMPLED_TEXTURE_ARRAY_AND_SAMPLER_LAYOUT_DESC,
        ];
        let bind_group_layouts = bind_group_layout_desc
            .iter()
            .map(|desc| &graphics.device.create_bind_group_layout(desc)).collect::<Vec<_>>().as_slice();

        let vertex_buffer_layouts = &[Vertex::init_buffer_layout(), InstanceRaw::init_buffer_layout()];

        let main_pipeline = Pipeline::new(
            "main",
            &graphics,
            v_path,
            f_path,
            Some(Texture::DEPTH_FORMAT),
            bind_group_layouts,
            vertex_buffer_layouts,
        );
        self.pipelines.push(main_pipeline);
    }

    pub fn add_custom_pipeline(&mut self, pipeline: Pipeline) {
        self.pipelines.push(pipeline);
    }
}
