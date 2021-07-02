use crate::texture::Texture;
use crate::uniform_matrix::MatrixUniform;
use crate::{
    camera::{Camera, CameraController},
    texture,
    vertex_index::{Vertex, VertexLayout},
};
use nalgebra::Point3;
use wgpu::util::DeviceExt;
use crate::model::Mesh;

pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    pub size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    model: Mesh,
    model_uniform: (wgpu::Buffer, wgpu::BindGroupLayout, wgpu::BindGroup),
    clear: wgpu::Color,
    camera: Camera,
    camera_controller: CameraController,
    matrix_uniform: MatrixUniform,
    depth_texture: Texture,
}

const VERTICES: &[Vertex] = &[
    Vertex {
        //br
        position: [1.0, -1.0, -1.0],
        color: [0.5, 0.0, 0.0],
    },
    Vertex {
        //tl
        position: [-1.0, 1.0, -1.0],
        color: [0.0, 0.5, 0.0],
    },
    Vertex {
        //bl
        position: [-1.0, -1.0, -1.0],
        color: [0.0, 0.0, 0.5],
    },
    Vertex {
        //tr
        position: [1.0, 1.0, -1.0],
        color: [0.5, 0.0, 0.0],
    },
    Vertex {
        //br    4
        position: [1.0, -1.0, -3.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        //tr
        position: [1.0, 1.0, -3.0],
        color: [0.0, 0.0, 0.5],
    },
    Vertex {
        //bl
        position: [-1.0, -1.0, -3.0],
        color: [0.5, 0.0, 0.0],
    },
    Vertex {
        //tl
        position: [-1.0, 1.0, -3.0],
        color: [0.0, 0.5, 0.0],
    },
];

const INDICES: &[u16] = &[
    0, 1, 2, 0, 3, 1, 0, 5, 3, 0, 4, 5, 4, 6, 5, 5, 6, 7, 6, 2, 1, 6, 1, 7, 1, 3, 5, 5, 7, 1, 0, 2,
    4, 2, 6, 4,
];

impl State {
    pub async fn new(window: &winit::window::Window) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: adapter.get_swap_chain_preferred_format(&surface).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let vert_shader =
            device.create_shader_module(&wgpu::include_spirv!("shaders/shader.vert.spv"));
        let frag_shader =
            device.create_shader_module(&wgpu::include_spirv!("shaders/shader.frag.spv"));

        let mut camera = Camera::new(
            Point3::new(0., 0., 2.),
            Point3::new(0., 0., 0.),
            &sc_desc,
            45.,
        );
        let camera_controller = CameraController::new();

        let mut matrix_uniform = MatrixUniform::new(&device, &camera);
        matrix_uniform.update_uniform(&mut camera);

        let model = Mesh::custom_mesh("Cube", &device, VERTICES, INDICES);
        let model_uniform = ModelUniform::new_uniform("Model Uniform", &device, 1);

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&matrix_uniform.bind_group_layout, &model_uniform.bind_group_layout],
                push_constant_ranges: &[],
            });

        let depth_texture =
            texture::Texture::create_depth_texture(&device, &sc_desc, "depth_texture");

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vert_shader,
                entry_point: "main",
                buffers: &[Vertex::init_buffer_layout(), InstanceUniformRaw::init_buffer_layout()],
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
                format: texture::Texture::DEPTH_FORMAT,
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
                module: &frag_shader,
                entry_point: "main",
                targets: &[wgpu::ColorTargetState {
                    format: sc_desc.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrite::ALL,
                }],
            }),
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsage::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsage::INDEX,
        });

        let clear = wgpu::Color {
            r: 0.1,
            g: 0.2,
            b: 0.3,
            a: 1.0,
        };

        State {
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            size,
            render_pipeline,
            model,
            model_uniform,
            clear,
            camera,
            camera_controller,
            matrix_uniform,
            depth_texture,
        }
    }

    pub fn update(&mut self) {
        self.camera_controller.update_camera(&mut self.camera);
        self.matrix_uniform.update_uniform(&mut self.camera);
        self.queue.write_buffer(
            &self.matrix_uniform.buffer,
            0,
            bytemuck::cast_slice(&[self.matrix_uniform.proj_view_model_matrix]),
        );
    }

    pub fn render(&self) -> Result<(), wgpu::SwapChainError> {
        let frame = self.swap_chain.get_current_frame()?.output;
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Command Encoder"),
            });
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &frame.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(self.clear),
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

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.matrix_uniform.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.model.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.model.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        render_pass.draw_indexed(0..self.model.index_length as u32, 0, 0..1);

        drop(render_pass);

        self.queue.submit(std::iter::once(encoder.finish()));

        Ok(())
    }

    pub fn input(&mut self, event: &winit::event::DeviceEvent) {
        self.camera_controller.process_input(event);
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.depth_texture =
            texture::Texture::create_depth_texture(&self.device, &self.sc_desc, "depth_texture");
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }
}
